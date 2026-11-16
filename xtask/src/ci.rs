//! `cargo xtask ci`: every check of CI, in the order that CI runs them.

use std::path::Path;
use std::process::Command;

use crate::{Error, Result};

/// Keep this list the same as the checks in the "Development" section of `README.md`.
const STEPS: &[&str] = &[
    "cargo fmt --check",
    "ast-grep test -c lint/ast-grep/sgconfig.yml --skip-snapshot-tests",
    "ast-grep scan -c lint/ast-grep/sgconfig.yml src",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo nextest run --workspace --profile ci",
    "node --test .claude/skills/port-status/port-status.test.ts",
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
