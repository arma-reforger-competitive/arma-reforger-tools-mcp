//! The MCP server: one Tool router with the Tools of every Capability.

use std::fmt::Display;
use std::sync::Arc;

use rmcp::{
    ErrorData, RoleServer, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, tool::ToolCallContext},
    model::{
        CallToolRequestParams, CallToolResponse, CallToolResult, ContentBlock, Implementation,
        JsonObject, ServerCapabilities, ServerConfig, ToolAnnotations,
    },
    service::RequestContext,
    tool_handler,
    transport::stdio,
};

use crate::{answer_size, api_docs, game_files, guid, reference_mods, wiki, workbench};

/// The Server guide: Claude Code reads it once when a session starts. It names every Tool in
/// backticks, and a test in `tests/server.rs` checks that each name is a served Tool.
const SERVER_GUIDE: &str =
    "Arma Reforger modding help. The script API, the wiki, and the game's own \
files are on this computer, so these Tools answer fast and need no internet.

- `api_search`: classes and methods of the Enfusion and Arma Reforger script API, with \
signatures and parent classes.
- `wiki_search`: find how-to pages of the Arma Reforger wiki and the Enfusion engine pages. \
`wiki_read`: read one page by its `title`. When the user names the page, call `wiki_read` \
directly.
- `game_find`: the scripts, Prefabs, and configs of the game and of the Workshop mods that \
Workbench downloaded. They are inside .pak archives, so grep, find, and file search cannot see \
them, and they are not in the project folder.
- `game_read`: open one file of the game or of a downloaded mod by its `path`, and get its full \
text. When the user gives the path of a file of the game or of a Workshop mod, call `game_read` \
directly with that path, not Read or find.
- `workbench_status`: the live state of the user's running Workbench: whether it runs, whether \
the Handler script of this plugin is loaded, and which mod is open. For how to do something in \
Workbench, use the wiki Tools.
- `workbench_launch`: start Workbench with the user's mod named in `mod`, and wait until it is \
ready. When Workbench runs already, it starts nothing and names the open mod.
- `reference_sync`: get, update, or clean up the Reference mods: the Workshop mods that the \
user's mod repos list for study. It downloads the missing ones and deletes the dropped ones. It \
finds and reads the lists itself, and they are not in the project folder, so call it directly, \
with no input, and do not look for the lists first.
- `guid_generate`: make a new GUID for the .gproj file of a new mod. It takes no input. Do not \
make up a GUID or write a script for one.

A class name, a method signature, or a game file path from memory is often wrong. These Tools \
give the real one.";

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
#[tool_handler(router = self.tools)]
impl ServerHandler for Server {
    /// The server's name, its Tools, and the Server guide.
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("reforger", env!("CARGO_PKG_VERSION")))
            .with_instructions(SERVER_GUIDE)
    }

    /// Runs one Tool call and cuts a long answer. When Claude cancels the call, the Tool stops at
    /// its next `await`.
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
            response = call => response.map(cut_answer),
            () = cancelled.cancelled() => {
                Err(ErrorData::internal_error("Claude cancelled the call", None))
            }
        }
    }
}

/// Serves the Tools of every Capability over stdio until Claude Code closes the connection.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server::new(
        wiki::tools()?
            + api_docs::tools()?
            + game_files::tools()
            + workbench::tools()
            + reference_mods::tools()
            + guid::tools(),
    );
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}

/// The text of a Tool answer, or the hint of an error answer for a wrong input from Claude.
// Reason: a hint is a text for Claude, not a Rust error (docs/agents/rust.md, Errors).
// ast-grep-ignore: string-error
pub type Answer = Result<String, String>;

/// Turns the result of a Tool into its answer. `Ok` gives a normal answer with the text. `Err`
/// gives an error answer with the `Display` text of the error, which is the hint for Claude.
/// Every Tool answer comes from here, and the `tool-answer` check of ast-grep stops other ways.
pub fn answer<T: Into<String>, E: Display>(result: Result<T, E>) -> CallToolResult {
    match result {
        Ok(text) => CallToolResult::success(vec![ContentBlock::text(text)]),
        Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
    }
}

/// The input schema of a Tool with no inputs. MCP needs `"type": "object"`, and a client drops a
/// Tool whose schema does not have it.
pub fn no_inputs() -> Arc<JsonObject> {
    let mut schema = JsonObject::new();
    schema.insert("type".to_owned(), "object".into());
    Arc::new(schema)
}

/// The Tool labels of a Tool that only reads files on this computer: it changes nothing, a second
/// call gives the same answer, and it does not use the internet.
pub fn read_only_labels() -> ToolAnnotations {
    ToolAnnotations::new()
        .read_only(true)
        .destructive(false)
        .idempotent(true)
        .open_world(false)
}

// region:    --- Support

/// Cuts a long answer. A task or a request for input has no answer text yet, so it stays as it is.
fn cut_answer(response: CallToolResponse) -> CallToolResponse {
    match response {
        CallToolResponse::Complete(answer) => CallToolResponse::Complete(answer_size::cut(answer)),
        other => other,
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use rmcp::model::{CallToolRequest, ClientRequest, Tool};
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
