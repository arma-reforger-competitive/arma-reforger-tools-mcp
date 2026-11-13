use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, ClientConfig},
    transport::TokioChildProcess,
};
use tokio::process::Command;

#[tokio::test]
async fn starts_the_reforger_server_over_stdio_and_answers_wiki_read() {
    let server = TokioChildProcess::new(Command::new(env!("CARGO_BIN_EXE_reforger"))).unwrap();
    let client = ClientConfig::default().serve(server).await.unwrap();

    let server_info = client.peer_info().unwrap().server_info.clone().unwrap();
    assert_eq!(server_info.name, "reforger");
    let arguments = serde_json::json!({ "title": "Arma Reforger:Mod Project Setup" });
    let result = client
        .call_tool(
            CallToolRequestParams::new("wiki_read")
                .with_arguments(arguments.as_object().unwrap().clone()),
        )
        .await
        .unwrap();
    let text = &result.content[0]
        .as_text()
        .expect("wiki_read did not answer with text")
        .text;
    assert!(text.contains("## Arma Reforger:Mod Project Setup"));

    client.cancel().await.unwrap();
}
