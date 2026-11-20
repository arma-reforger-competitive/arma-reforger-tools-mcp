//! The Tool of the guid Capability.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{CallToolResult, ContentBlock, Tool, ToolAnnotations};

use super::{Error, Result};
use crate::server::no_inputs;

/// Gives the guid Tool to the server.
pub fn tools<S: Send + Sync + 'static>() -> ToolRouter<S> {
    let guid_generate = Tool::new(
        "guid_generate",
        "Make a new GUID for the .gproj file of a new mod project. The answer is one GUID only: \
         16 hex characters in upper case, for example 0123456789ABCDEF. The GUID is the ID of \
         the mod: other mods name it in their dependencies, and the Workshop uses it as the \
         mod's ID, so each mod needs its own GUID. Each call gives a different GUID, made from \
         random bytes of the operating system. When the operating system gives no random bytes, \
         the answer is an error that says to call again.",
        no_inputs(),
    )
    .with_title("Make a mod GUID")
    .annotate(labels());
    ToolRouter::new().with_route((guid_generate, || async {
        match generate() {
            Ok(guid) => CallToolResult::success(vec![ContentBlock::text(guid)]),
            Err(error) => CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
        }
    }))
}

/// Makes a GUID from 8 random bytes of the operating system, written as 16 hex characters in
/// upper case. `{:016X}` keeps the leading zeros.
fn generate() -> Result<String> {
    let mut bytes = [0_u8; 8];
    getrandom::fill(&mut bytes).map_err(Error::NoRandomBytes)?;
    Ok(format!("{:016X}", u64::from_be_bytes(bytes)))
}

// region:    --- Support

/// It changes nothing and does not use the internet. A second call gives a new GUID, so it is not
/// idempotent.
fn labels() -> ToolAnnotations {
    ToolAnnotations::new()
        .read_only(true)
        .destructive(false)
        .idempotent(false)
        .open_world(false)
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::test_support;

    #[tokio::test]
    async fn returns_16_hex_characters_in_upper_case() {
        // -- Setup & Fixtures
        let client = test_support::connect(tools()).await;

        // -- Exec
        let answer = test_support::call(&client, "guid_generate", json!({})).await;

        // -- Check
        assert!(!answer.is_error, "{}", answer.text);
        assert_eq!(answer.text.len(), 16, "{}", answer.text);
        assert!(
            answer
                .text
                .chars()
                .all(|c| matches!(c, '0'..='9' | 'A'..='F')),
            "{}",
            answer.text
        );
    }

    #[tokio::test]
    async fn returns_a_different_guid_on_each_call() {
        // -- Setup & Fixtures
        let client = test_support::connect(tools()).await;

        // -- Exec
        let first = test_support::call(&client, "guid_generate", json!({})).await;
        let second = test_support::call(&client, "guid_generate", json!({})).await;

        // -- Check
        assert!(!first.is_error, "{}", first.text);
        assert!(!second.is_error, "{}", second.text);
        assert_ne!(first.text, second.text);
    }
}

// endregion: --- Tests
