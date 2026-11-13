use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, ClientConfig},
    transport::TokioChildProcess,
};
use serde_json::json;
use tokio::process::Command;

#[tokio::test]
async fn starts_the_reforger_server_over_stdio_and_answers_wiki_read() {
    // -- Setup & Fixtures
    let server = TokioChildProcess::new(Command::new(env!("CARGO_BIN_EXE_reforger"))).unwrap();
    let client = ClientConfig::default().serve(server).await.unwrap();

    // -- Exec
    let server_info = client.peer_info().unwrap().server_info.clone().unwrap();
    let arguments = json!({ "title": "Arma Reforger:Mod Project Setup" });
    let result = client
        .call_tool(
            CallToolRequestParams::new("wiki_read")
                .with_arguments(arguments.as_object().unwrap().clone()),
        )
        .await
        .unwrap();

    // -- Check
    assert_eq!(server_info.name, "reforger");
    let text = &result.content[0]
        .as_text()
        .expect("wiki_read did not answer with text")
        .text;
    assert!(text.contains("## Arma Reforger:Mod Project Setup"));

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn describes_every_tool_and_every_tool_input() {
    // -- Setup & Fixtures
    let server = TokioChildProcess::new(Command::new(env!("CARGO_BIN_EXE_reforger"))).unwrap();
    let client = ClientConfig::default().serve(server).await.unwrap();

    // -- Exec & Check
    for tool in client.list_all_tools().await.unwrap() {
        let description = tool.description.as_deref().unwrap_or_default();
        assert!(
            !description.trim().is_empty(),
            "{} has no description",
            tool.name
        );
        let inputs = tool
            .input_schema
            .get("properties")
            .and_then(serde_json::Value::as_object)
            .into_iter()
            .flatten();
        for (input, schema) in inputs {
            let description = schema
                .get("description")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default();
            assert!(
                !description.trim().is_empty(),
                "{} has no description for the input {input}",
                tool.name
            );
        }
    }

    client.cancel().await.unwrap();
}
