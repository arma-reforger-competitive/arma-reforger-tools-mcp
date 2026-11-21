//! The wire format of the Workbench NET API (the wiki page "Workbench NET API").
//!
//! A request is the protocol version (a 4-byte little-endian integer), then three Pascal strings:
//! the client ID, the content type `JsonRPC`, and the JSON payload. An answer is two Pascal
//! strings: the status (`Ok` or an error text) and the JSON payload. A Pascal string is a 4-byte
//! little-endian length and then that many UTF-8 bytes.
//!
//! This file uses only `std`, because `cargo xtask net-api` builds it too.

use std::fmt;

/// The only protocol version that Workbench knows.
const PROTOCOL_VERSION: i32 = 1;

/// The name that Workbench sees for this client.
const CLIENT_ID: &str = "reforger";

/// The only content type that Workbench knows.
const CONTENT_TYPE: &str = "JsonRPC";

/// The longest answer that a client reads. A longer one is not a NET API answer.
pub(super) const MAX_ANSWER_BYTES: usize = 16 * 1024 * 1024;

/// The bytes of one request with the JSON `payload`.
pub(super) fn request(payload: &str) -> Vec<u8> {
    let mut bytes = PROTOCOL_VERSION.to_le_bytes().to_vec();
    for text in [CLIENT_ID, CONTENT_TYPE, payload] {
        push_string(&mut bytes, text);
    }
    bytes
}

/// The status and the payload of a whole answer.
pub(super) fn answer(bytes: &[u8]) -> Result<(String, String), BadBytes> {
    let (status, rest) = read_string(bytes)?;
    // An error status can come with no payload.
    if rest.is_empty() {
        return Ok((status, String::new()));
    }
    let (payload, rest) = read_string(rest)?;
    if !rest.is_empty() {
        return Err(BadBytes::ExtraBytes { count: rest.len() });
    }
    Ok((status, payload))
}

/// Adds `text` as a Pascal string.
pub(super) fn push_string(bytes: &mut Vec<u8>, text: &str) {
    let len = u32::try_from(text.len()).unwrap_or(u32::MAX);
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(text.as_bytes());
}

/// Why the bytes of an answer are not a whole answer.
#[derive(Debug)]
pub enum BadBytes {
    /// The answer ends in the middle of a string, or is empty.
    Ends,
    ExtraBytes {
        count: usize,
    },
    NotUtf8,
}

impl fmt::Display for BadBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ends => write!(f, "it ends in the middle of a string"),
            Self::ExtraBytes { count } => write!(f, "it has {count} bytes after the payload"),
            Self::NotUtf8 => write!(f, "a string in it is not UTF-8"),
        }
    }
}

// region:    --- Support

/// Reads one Pascal string from the start of `bytes` and returns it with the bytes after it.
fn read_string(bytes: &[u8]) -> Result<(String, &[u8]), BadBytes> {
    let (len, rest) = bytes.split_first_chunk::<4>().ok_or(BadBytes::Ends)?;
    let len = usize::try_from(u32::from_le_bytes(*len)).map_err(|_| BadBytes::Ends)?;
    let (text, rest) = rest.split_at_checked(len).ok_or(BadBytes::Ends)?;
    let text = String::from_utf8(text.to_vec()).map_err(|_| BadBytes::NotUtf8)?;
    Ok((text, rest))
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_a_request_as_the_wiki_shows_it() {
        // -- Exec
        let bytes = request("{}");

        // -- Check
        let mut expected = vec![1, 0, 0, 0];
        expected.extend_from_slice(&[8, 0, 0, 0]);
        expected.extend_from_slice(b"reforger");
        expected.extend_from_slice(&[7, 0, 0, 0]);
        expected.extend_from_slice(b"JsonRPC");
        expected.extend_from_slice(&[2, 0, 0, 0]);
        expected.extend_from_slice(b"{}");
        assert_eq!(bytes, expected);
    }

    #[test]
    fn counts_the_length_of_a_string_in_bytes_not_in_chars() {
        // -- Exec
        let bytes = request("é");

        // -- Check
        // "é" is 2 bytes in UTF-8.
        assert_eq!(
            bytes.get(bytes.len() - 6..),
            Some(&[2, 0, 0, 0, 0xc3, 0xa9][..])
        );
    }

    #[test]
    fn reads_an_error_status_with_no_payload_as_a_whole_answer() {
        // -- Exec
        let (status, payload) = answer(b"\x12\x00\x00\x00Undefined API func").unwrap();

        // -- Check
        assert_eq!(status, "Undefined API func");
        assert_eq!(payload, "");
    }

    #[test]
    fn reads_the_answer_that_workbench_gave_to_get_loaded_projects() {
        // -- Setup & Fixtures
        // The bytes that Workbench 1.6 sent to GetLoadedProjects with the BridgeTest mod open.
        let bytes = b"\x02\x00\x00\x00Ok1\x00\x00\x00{\"Loaded Projects\":[\"ArmaReforger\",\"BridgeTest\"]}";

        // -- Exec
        let (status, payload) = answer(bytes).unwrap();

        // -- Check
        assert_eq!(status, "Ok");
        assert_eq!(
            payload,
            r#"{"Loaded Projects":["ArmaReforger","BridgeTest"]}"#
        );
    }

    #[test]
    fn rejects_an_answer_that_ends_too_early() {
        // -- Exec & Check
        for bytes in [
            &b""[..],
            b"\x02\x00",
            b"\x09\x00\x00\x00Ok",
            b"\x02\x00\x00\x00Ok\x05\x00",
        ] {
            assert!(matches!(answer(bytes), Err(BadBytes::Ends)), "{bytes:?}");
        }
    }

    #[test]
    fn rejects_bytes_after_the_payload() {
        // -- Exec & Check
        assert!(matches!(
            answer(b"\x02\x00\x00\x00Ok\x02\x00\x00\x00{}xyz"),
            Err(BadBytes::ExtraBytes { count: 3 })
        ));
    }
}

// endregion: --- Tests
