mod wiki;

use rmcp::{
    ServerHandler, ServiceExt, handler::server::router::tool::ToolRouter, tool_handler,
    transport::stdio,
};

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

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Server {
        tools: wiki::tools()?,
    };
    server.serve(stdio()).await?.waiting().await?;
    Ok(())
}
