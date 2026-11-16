//! The MCP server: one Tool router with the Tools of every Capability.

use rmcp::{
    ErrorData, RoleServer, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, tool::ToolCallContext},
    model::{CallToolRequestParams, CallToolResponse},
    service::RequestContext,
    tool_handler,
    transport::stdio,
};

use crate::wiki;

#[derive(Clone)]
pub struct Server {
    tools: ToolRouter<Self>,
}

impl Server {
    pub fn new(tools: ToolRouter<Self>) -> Self {
        Self { tools }
    }
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "the rmcp tool_handler macro writes list_tools as async"
)]
#[tool_handler(router = self.tools, name = "reforger")]
impl ServerHandler for Server {
    /// Runs one Tool call. When Claude cancels the call, the Tool stops at its next `await`.
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let cancelled = context.ct.clone();
        let call = self
            .tools
            .call(ToolCallContext::new(self, request, context));
        tokio::select! {
            response = call => response,
            () = cancelled.cancelled() => Err(ErrorData::internal_error("Claude cancelled the call", None)),
        }
    }
}

/// Serves the Tools of every Capability over stdio until Claude Code closes the connection.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(wiki::tools()?);
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use rmcp::model::{CallToolRequest, CallToolResult, ClientRequest, Tool};
    use rmcp::service::PeerRequestOptions;
    use serde_json::Map;
    use tokio::sync::Notify;

    use super::*;
    use crate::test_support;

    #[tokio::test]
    async fn stops_a_tool_when_claude_cancels_the_call() {
        // -- Setup & Fixtures
        let started = Arc::new(Notify::new());
        let stopped = Arc::new(Notify::new());
        let client = test_support::connect(wait_forever_tool(&started, &stopped)).await;
        let request = ClientRequest::CallToolRequest(CallToolRequest::new(
            CallToolRequestParams::new("wait_forever"),
        ));

        // -- Exec
        let call = client
            .send_cancellable_request(request, PeerRequestOptions::no_options())
            .await
            .unwrap();
        started.notified().await;
        call.cancel(Some("the user pressed Esc".into()))
            .await
            .unwrap();

        // -- Check
        tokio::time::timeout(Duration::from_secs(5), stopped.notified())
            .await
            .expect("the Tool still runs 5 seconds after the cancel");
    }

    // -- Test Support

    /// A Tool that never ends. It signals `started` when it runs and `stopped` when it is dropped.
    fn wait_forever_tool(started: &Arc<Notify>, stopped: &Arc<Notify>) -> ToolRouter<Server> {
        let tool = Tool::new("wait_forever", "Waits until it is cancelled.", Map::new());
        let (started, stopped) = (Arc::clone(started), Arc::clone(stopped));
        ToolRouter::new().with_route((tool, move || {
            let guard = SignalOnDrop(Arc::clone(&stopped));
            started.notify_one();
            async move {
                let _guard = guard;
                std::future::pending::<CallToolResult>().await
            }
        }))
    }

    struct SignalOnDrop(Arc<Notify>);

    impl Drop for SignalOnDrop {
        fn drop(&mut self) {
            self.0.notify_one();
        }
    }
}

// endregion: --- Tests
