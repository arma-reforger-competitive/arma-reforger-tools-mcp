//! Paths in Windows form and in WSL form (ADR 0006). A setting can hold either form, and the
//! server turns it into the form of the computer it runs on.

use std::path::PathBuf;

/// The path of the folder `raw` on this computer. In WSL, `D:\Games` becomes `/mnt/d/Games`. On
/// Windows, `/mnt/d/Games` becomes `D:\Games`. Spaces and quotes around the value are removed.
pub fn host_path(raw: &str) -> PathBuf {
    let raw = raw.trim().trim_matches(['"', '\'']);
    let converted = if cfg!(windows) {
        windows_from_wsl(raw)
    } else {
        wsl_from_windows(raw)
    };
    PathBuf::from(converted.unwrap_or_else(|| raw.to_owned()))
}

// region:    --- Support

/// `D:\Games` or `D:/Games` as `/mnt/d/Games`, or `None` when `path` does not start with a drive.
fn wsl_from_windows(path: &str) -> Option<String> {
    let mut chars = path.chars();
    let drive = chars.next().filter(char::is_ascii_alphabetic)?;
    let rest = chars.as_str().strip_prefix(':')?;
    if !(rest.is_empty() || rest.starts_with(['\\', '/'])) {
        return None;
    }
    Some(format!(
        "/mnt/{}{}",
        drive.to_ascii_lowercase(),
        rest.replace('\\', "/")
    ))
}

/// `/mnt/d/Games` as `D:\Games`, or `None` when `path` is not under `/mnt/<drive>`.
fn windows_from_wsl(path: &str) -> Option<String> {
    let mut chars = path.strip_prefix("/mnt/")?.chars();
    let drive = chars.next().filter(char::is_ascii_alphabetic)?;
    let rest = chars.as_str();
    if !(rest.is_empty() || rest.starts_with('/')) {
        return None;
    }
    let rest = if rest.is_empty() { "\\" } else { rest };
    Some(format!(
        "{}:{}",
        drive.to_ascii_uppercase(),
        rest.replace('/', "\\")
    ))
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turns_a_windows_path_into_a_wsl_path() {
        // -- Exec & Check
        for (windows, wsl) in [
            (
                r"D:\Games\steamapps\common\Arma Reforger",
                "/mnt/d/Games/steamapps/common/Arma Reforger",
            ),
            (
                "C:/Users/me/Documents/My Games/ArmaReforgerWorkbench",
                "/mnt/c/Users/me/Documents/My Games/ArmaReforgerWorkbench",
            ),
            (r"E:\", "/mnt/e/"),
        ] {
            assert_eq!(wsl_from_windows(windows).as_deref(), Some(wsl));
        }
    }

    #[test]
    fn turns_a_wsl_path_into_a_windows_path() {
        // -- Exec & Check
        for (wsl, windows) in [
            (
                "/mnt/d/Games/steamapps/common/Arma Reforger",
                r"D:\Games\steamapps\common\Arma Reforger",
            ),
            ("/mnt/c", r"C:\"),
        ] {
            assert_eq!(windows_from_wsl(wsl).as_deref(), Some(windows));
        }
    }

    #[test]
    fn keeps_a_path_that_has_no_drive() {
        // -- Exec & Check
        for path in ["/home/me/game", "relative/game", "/mnt/data/game", "DD:\\x"] {
            assert_eq!(wsl_from_windows(path), None, "{path}");
            assert_eq!(windows_from_wsl(path), None, "{path}");
        }
    }

    #[test]
    fn removes_quotes_and_spaces_around_the_value() {
        // -- Exec & Check
        assert_eq!(
            host_path("  \"/home/me/game\" "),
            PathBuf::from("/home/me/game")
        );
    }
}

// endregion: --- Tests
