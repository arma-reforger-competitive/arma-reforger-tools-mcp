//! Helpers for the tests of every Capability: an in-memory MCP server and client.

use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::router::tool::ToolRouter,
    model::{CallToolRequestParams, ClientConfig},
    service::{RoleClient, RunningService},
    tool_handler,
};
use serde_json::Value;

/// An MCP client that is connected to a `TestServer`.
pub type TestClient = RunningService<RoleClient, ClientConfig>;

/// The text and the error flag of one Tool answer.
pub struct Answer {
    pub text: String,
    pub is_error: bool,
}

/// A server that has only the Tools that a test gives it.
#[derive(Clone)]
pub struct TestServer {
    tools: ToolRouter<Self>,
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "the rmcp tool_handler macro writes list_tools as async"
)]
#[tool_handler(router = self.tools)]
impl ServerHandler for TestServer {}

/// Serves `tools` on an in-memory server and connects a client to it.
pub async fn connect(tools: ToolRouter<TestServer>) -> TestClient {
    let (server_side, client_side) = tokio::io::duplex(1 << 20);
    let server = TestServer { tools };
    tokio::spawn(async move { server.serve(server_side).await.unwrap().waiting().await });
    ClientConfig::default().serve(client_side).await.unwrap()
}

/// Calls `tool` with `arguments` and returns its answer, which must be one text block.
pub async fn call(client: &TestClient, tool: &'static str, arguments: Value) -> Answer {
    let arguments = arguments.as_object().unwrap().clone();
    let result = client
        .call_tool(CallToolRequestParams::new(tool).with_arguments(arguments))
        .await
        .unwrap();
    let [content] = result.content.as_slice() else {
        panic!("{tool} did not answer with one content block");
    };
    Answer {
        text: content
            .as_text()
            .expect("the Tool did not answer with text")
            .text
            .clone(),
        is_error: result.is_error.unwrap_or(false),
    }
}
