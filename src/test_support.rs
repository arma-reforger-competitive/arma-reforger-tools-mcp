//! Helpers for the tests of every Capability: the real server in memory, and a client.

use rmcp::{
    ServiceExt,
    handler::server::router::tool::ToolRouter,
    model::{CallToolRequestParams, ClientConfig},
    service::{RoleClient, RunningService},
};
use serde_json::Value;

use crate::server::Server;

/// An MCP client that is connected to an in-memory `Server`.
pub type TestClient = RunningService<RoleClient, ClientConfig>;

/// The text and the error flag of one Tool answer.
pub struct Answer {
    pub text: String,
    pub is_error: bool,
}

/// Serves only `tools` on an in-memory `Server` and connects a client to it.
pub async fn connect(tools: ToolRouter<Server>) -> TestClient {
    let (server_side, client_side) = tokio::io::duplex(1 << 20);
    let server = Server::new(tools);
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
