//! The `reforger` MCP server for Arma Reforger modding.

// region:    --- Modules

mod answer_size;
#[path = "api-docs/mod.rs"]
mod api_docs;
#[path = "game-files/mod.rs"]
mod game_files;
mod host_path;
mod server;
mod wiki;
mod workbench;
mod workbench_documents;

#[cfg(test)]
mod test_support;

// endregion: --- Modules

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Logs go to stderr, because stdout carries the MCP messages. Claude Code saves them in
    // ~/.cache/claude-cli-nodejs/<project>/mcp-logs-plugin-reforger-reforger/.
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();
    server::run().await
}
