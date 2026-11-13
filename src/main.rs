//! The `reforger` MCP server for Arma Reforger modding.

// region:    --- Modules

mod server;
mod wiki;

#[cfg(test)]
mod test_support;

// endregion: --- Modules

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::run().await
}
