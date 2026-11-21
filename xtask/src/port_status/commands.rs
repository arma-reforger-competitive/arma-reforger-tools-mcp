//! The two commands, `report` and `set`, and all their output.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;

use super::files::read_text;
use super::ledger::{
    self, LinkKind, Status, check_ported, history_line, kind_id, link_to, parse_ledger,
    render_ledger, set_status, sync, verdict,
};
use super::old_project::{inventory, read_usage};
use super::{Error, Result};
use crate::cmd::{NewStatus, PortStatusCmd, PortStatusSetArgs};

const OLD_REPO_SETTING: &str = "PORT_STATUS_OLD_REPO";
const HISTORY_SETTING: &str = "PORT_STATUS_HISTORY";

/// The first history line of a ledger.
const HISTORY_LINE: &str = "(?mR)^History: [^\r\n\u{2028}\u{2029}]*";

pub fn run(command: &PortStatusCmd) -> Result<()> {
    let new_repo = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/.."));
    let ledger_path = new_repo.join("docs").join("port-ledger.md");
    match command {
        PortStatusCmd::Report => {
            let old_repo = setting(OLD_REPO_SETTING, &["www", "arma-enfusion-mcp"])?;
            let history_dir = setting(HISTORY_SETTING, &[".claude", "projects"])?;
            report(new_repo, &ledger_path, &old_repo, &history_dir)
        }
        PortStatusCmd::Set(args) => set(new_repo, &ledger_path, args),
    }
}

// region:    --- Support

fn report(new_repo: &Path, ledger_path: &Path, old_repo: &Path, history_dir: &Path) -> Result<()> {
    if !old_repo.join("src").exists() {
        return Err(Error::NoOldProject {
            path: old_repo.to_owned(),
        });
    }
    let features = inventory(old_repo)?;
    let usage = read_usage(history_dir, &features)?;
    let before = if ledger_path.exists() {
        parse_ledger(&read_text(ledger_path)?)?
    } else {
        Vec::new()
    };
    let synced = sync(before, &features, &usage);
    let rows = synced.rows;
    write(ledger_path, &render_ledger(&rows, &history_line(&usage)))?;
    let problems = check_ported(&rows, new_repo);

    println!("{}", history_line(&usage));
    if !synced.added.is_empty() {
        let added: Vec<String> = synced
            .added
            .iter()
            .map(|f| format!("{}:{}", kind_id(f.kind), f.name))
            .collect();
        println!("\nAdded to the ledger: {}", added.join(", "));
    }
    if !synced.stale.is_empty() {
        let stale: Vec<String> = synced
            .stale
            .iter()
            .map(|r| format!("{}:{}", kind_id(r.kind), r.name))
            .collect();
        println!(
            "\nIn the ledger, but not in the old project: {}",
            stale.join(", ")
        );
    }
    if !problems.is_empty() {
        println!("\n\"Ported\" rows that do not lead to a Tool or skill:");
        for ledger::Problem { row, problem } in &problems {
            println!("- {}:{} {problem}", kind_id(row.kind), row.name);
        }
    }
    let open: Vec<&ledger::Row> = rows.iter().filter(|r| r.status == Status::NotYet).collect();
    if !open.is_empty() {
        println!("\nNot yet decided:");
        for row in open {
            let ticket = if row.detail.is_empty() {
                String::new()
            } else {
                format!(" (ticket: {})", row.detail)
            };
            println!(
                "- {}:{}: {}{ticket}",
                kind_id(row.kind),
                row.name,
                row.suggestion
            );
        }
    }
    println!("\n{}", verdict(&rows, &problems));
    Ok(())
}

fn set(new_repo: &Path, ledger_path: &Path, args: &PortStatusSetArgs) -> Result<()> {
    let (status, detail) = match args.status {
        NewStatus::NotNeeded => (Status::NotNeeded, args.reason.clone().unwrap_or_default()),
        NewStatus::NotYet => (Status::NotYet, args.ticket.clone().unwrap_or_default()),
        NewStatus::Ported => {
            let detail = match (&args.tool, &args.skill) {
                (Some(tool), _) => link_to(new_repo, LinkKind::Tool, tool)?,
                (None, Some(skill)) => link_to(new_repo, LinkKind::Skill, skill)?,
                (None, None) => String::new(),
            };
            (Status::Ported, detail)
        }
    };
    if !ledger_path.exists() {
        return Err(Error::NoLedger);
    }
    let text = read_text(ledger_path)?;
    let changed = set_status(&parse_ledger(&text)?, &args.selectors, status, &detail)?;
    // Keep the history line of the last report.
    let history_line = Regex::new(HISTORY_LINE).map_err(|source| Error::BadRegex {
        pattern: HISTORY_LINE,
        source,
    })?;
    let history = history_line.find(&text).map_or("", |found| found.as_str());
    write(ledger_path, &render_ledger(&changed.rows, history))?;
    let names: Vec<String> = changed
        .changed
        .iter()
        .map(|r| format!("{}:{}", kind_id(r.kind), r.name))
        .collect();
    println!(
        "Marked {} rows as \"{status}\": {}",
        changed.changed.len(),
        names.join(", ")
    );
    Ok(())
}

/// The folder in the setting `name`, or the folder `default` under the home folder.
fn setting(name: &str, default: &[&str]) -> Result<PathBuf> {
    if let Some(value) = env::var_os(name) {
        return Ok(PathBuf::from(value));
    }
    let home = env::home_dir().ok_or(Error::NoHome)?;
    Ok(default.iter().fold(home, |path, part| path.join(part)))
}

fn write(path: &Path, text: &str) -> Result<()> {
    fs::write(path, text).map_err(|source| Error::Write {
        path: path.to_owned(),
        source,
    })
}

// endregion: --- Support
