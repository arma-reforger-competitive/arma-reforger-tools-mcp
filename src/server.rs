//! The MCP server: one Tool router with the Tools of every Capability.

use rmcp::{
    ServerHandler, ServiceExt, handler::server::router::tool::ToolRouter, tool_handler,
    transport::stdio,
};

use crate::wiki;

#[derive(Clone)]
struct Server {
    tools: ToolRouter<Self>,
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "the rmcp tool_handler macro writes list_tools as async"
)]
#[tool_handler(router = self.tools, name = "reforger")]
impl ServerHandler for Server {}

/// Serves the Tools of every Capability over stdio until Claude Code closes the connection.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server {
        tools: wiki::tools()?,
    };
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}
