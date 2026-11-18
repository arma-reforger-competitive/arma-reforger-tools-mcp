//! `cargo xtask ci`: every check of CI, in the order that CI runs them.

use std::path::Path;
use std::process::Command;

use crate::{Error, Result};

/// The checks in the "Development" section of `README.md`, in the same order. A test below
/// fails when the two lists differ.
const STEPS: &[&str] = &[
    "cargo fmt --check",
    "ast-grep test -c lint/ast-grep/sgconfig.yml --skip-snapshot-tests",
    "ast-grep scan -c lint/ast-grep/sgconfig.yml src",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo nextest run --workspace --profile ci",
    "node --test .claude/skills/port-status/port-status.test.ts",
    "node --test .claude/skills/workbench-test/net-api.test.ts",
    "cargo deny --workspace check",
];

pub fn run() -> Result<()> {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/.."));
    for &step in STEPS {
        eprintln!("\n=== ci: {step}");
        let mut words = step.split_whitespace();
        let Some(program) = words.next() else {
            continue;
        };
        let status = Command::new(program)
            .args(words)
            .current_dir(root)
            .status()
            .map_err(|source| Error::CiStepNotStarted { step, source })?;
        if !status.success() {
            return Err(Error::CiStepFailed { step, status });
        }
    }
    eprintln!("\nci: all {} steps passed.", STEPS.len());
    Ok(())
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_the_same_checks_as_the_list_in_the_readme() {
        // -- Setup & Fixtures
        let readme = include_str!("../../README.md");
        let after_intro = readme
            .split_once("runs these checks in this order")
            .expect("README.md has the sentence before the check list")
            .1;
        let block = after_intro
            .split_once("```sh\n")
            .and_then(|(_, rest)| rest.split_once("```"))
            .expect("README.md has an sh block after that sentence")
            .0;

        // -- Exec
        let readme_steps: Vec<&str> = block.lines().collect();

        // -- Check
        assert_eq!(readme_steps, STEPS);
    }
}

// endregion: --- Tests
