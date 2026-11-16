// Keeps the Port ledger (docs/port-ledger.md) up to date. SKILL.md in this folder says how to use it.
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { homedir } from "node:os";
import { join, relative, resolve } from "node:path";

export type Kind =
  | "tool"
  | "prompt"
  | "resource"
  | "template"
  | "recipe"
  | "pattern"
  | "knowledge"
  | "animation";
export type Status = "ported" | "not needed" | "not yet";
/** One feature of the old project. `uri` is the start of a resource's URI, for example "enfusion://class/". */
export type Feature = { kind: Kind; name: string; uri?: string };
export type Row = { kind: Kind; name: string; status: Status; detail: string; suggestion: string };
export type Count = { calls: number; errors: number; sessions: Set<string>; last: string };
/** Use of the old server in Claude Code session history. Keys of `counts` are "<kind>:<name>". */
export type Usage = { counts: Map<string, Count>; sessions: Set<string>; first: string; last: string };
export type Problem = { row: Row; problem: string };

const KINDS: { kind: Kind; heading: string }[] = [
  { kind: "tool", heading: "Tools" },
  { kind: "prompt", heading: "Prompts" },
  { kind: "resource", heading: "Resources" },
  { kind: "template", heading: "Templates" },
  { kind: "recipe", heading: "Recipes" },
  { kind: "pattern", heading: "Patterns" },
  { kind: "knowledge", heading: "Knowledge pages" },
  { kind: "animation", heading: "Animation graph code" },
];
const STATUSES: Status[] = ["ported", "not needed", "not yet"];
const OLD_SERVER = "enfusion-mcp";
const TOOL_PREFIX = `mcp__${OLD_SERVER}__`;

// region:    --- Old project

/** Lists every feature of the old project, read from its source files. */
export function inventory(oldRepo: string): Feature[] {
  const features: Feature[] = [];
  const add = (kind: Kind, names: Iterable<string>) => {
    for (const name of [...new Set(names)].sort()) features.push({ kind, name });
  };
  const source = filesUnder(join(oldRepo, "src"), ".ts")
    .map((file) => readFileSync(file, "utf8"))
    .join("\n");

  // Most tools call registerTool("name"). The Workbench tools give `name: "wb_..."` alone on a line.
  add("tool", [
    ...matches(source, /registerTool\(\s*"([a-z_]+)"/g),
    ...matches(source, /^\s*name: "([a-z_]+)",?$/gm),
  ]);
  add("prompt", matches(source, /registerPrompt\(\s*"([a-z-]+)"/g));
  for (const [, name, uri] of source.matchAll(
    /registerResource\(\s*"([a-z-]+)",\s*new ResourceTemplate\("([^"{]+)/g,
  ))
    features.push({ kind: "resource", name, uri });

  // Files with "recipe" in the name are the code of the recipes, not templates.
  const templates = join(oldRepo, "src", "templates");
  add(
    "template",
    filesUnder(templates, ".ts")
      .map((file) => withoutExtension(relative(templates, file)))
      .filter((name) => !name.includes("recipe")),
  );
  add("recipe", namesUnder(join(oldRepo, "data", "recipes"), ".json"));
  add("pattern", namesUnder(join(oldRepo, "data", "patterns"), ".json"));
  add("knowledge", namesUnder(join(oldRepo, "data", "kb", "patterns"), ".md"));
  if (existsSync(join(oldRepo, "src", "animation"))) add("animation", ["animation-graph-code"]);
  return features;
}

/**
 * The old tools that use a feature that Claude cannot call itself, read once from the imports
 * of the old code. The old repo is archived, so this list does not change. Undefined: not known.
 */
function consumers(feature: Feature): string[] | undefined {
  switch (feature.kind) {
    case "template":
      return {
        config: ["config_create"],
        gproj: ["mod"],
        layout: ["layout_create", "layout_recipe"],
        localization: [],
        prefab: ["prefab"],
        scenario: ["scenario_create"],
        script: ["script_create", "mod"],
        "server-config": ["server_config"],
      }[feature.name];
    case "recipe":
      if (feature.name.startsWith("gamemode/")) return ["gamemode_scaffold"];
      if (feature.name.startsWith("ui/")) return ["layout_recipe"];
      return ["prefab"];
    case "pattern":
      return ["mod"];
    case "knowledge":
      return ["wb_knowledge"];
    case "animation":
      return ["animation_graph"];
    default:
      return undefined;
  }
}

// endregion: --- Old project

// region:    --- Usage

/** Reads the calls to the old server's tools, prompts, and resources from Claude Code session history. */
export function readUsage(historyDir: string, features: Feature[]): Usage {
  const usage: Usage = { counts: new Map(), sessions: new Set(), first: "", last: "" };
  const resources = features.filter((feature) => feature.uri);
  const calls = new Map<string, { key: string; errors: boolean }>();
  const seen = new Set<string>();

  const count = (key: string, session: string, time: string) => {
    let entry = usage.counts.get(key);
    if (!entry) usage.counts.set(key, (entry = { calls: 0, errors: 0, sessions: new Set(), last: "" }));
    entry.calls++;
    entry.sessions.add(session);
    if (time > entry.last) entry.last = time;
  };

  for (const file of filesUnder(historyDir, ".jsonl")) {
    for (const line of readFileSync(file, "utf8").split("\n")) {
      if (!line.includes(OLD_SERVER) && !line.includes('"tool_result"')) continue;
      let record;
      try {
        record = JSON.parse(line);
      } catch {
        continue;
      }
      const session: string = record.sessionId ?? file;
      const time: string = record.timestamp ?? "";
      if (line.includes(OLD_SERVER)) {
        usage.sessions.add(session);
        if (time && (!usage.first || time < usage.first)) usage.first = time;
        if (time > usage.last) usage.last = time;
      }
      const content = record.message?.content;

      // An MCP prompt runs as a slash command.
      const text = typeof content === "string" ? content : "";
      const prompt = text.match(/<command-name>\/?(?:mcp__enfusion-mcp__|enfusion-mcp:)([a-z-]+)/);
      if (prompt && !seen.has(record.uuid)) {
        seen.add(record.uuid);
        count(`prompt:${prompt[1]}`, session, time);
      }

      if (!Array.isArray(content)) continue;
      for (const block of content) {
        if (block.type === "tool_use" && !calls.has(block.id)) {
          let key: string | undefined;
          if (block.name?.startsWith(TOOL_PREFIX)) key = `tool:${block.name.slice(TOOL_PREFIX.length)}`;
          if (block.name === "ReadMcpResourceTool" && block.input?.server === OLD_SERVER) {
            const resource = resources.find((feature) => block.input.uri?.startsWith(feature.uri));
            if (resource) key = `resource:${resource.name}`;
          }
          if (!key) continue;
          calls.set(block.id, { key, errors: false });
          count(key, session, time);
        }
        if (block.type === "tool_result" && block.is_error === true) {
          const call = calls.get(block.tool_use_id);
          if (call && !call.errors) {
            call.errors = true;
            usage.counts.get(call.key)!.errors++;
          }
        }
      }
    }
  }
  return usage;
}

/** What the usage suggests for one feature. Only a suggestion: the owner decides. */
export function suggest(feature: Feature, usage: Usage): string {
  const sessions = usage.sessions.size;
  const tools = consumers(feature);
  if (tools === undefined) {
    const used = usage.counts.get(`${feature.kind}:${feature.name}`);
    if (!used) return `never used in ${sessions} sessions, suggest: not needed`;
    return (
      `used ${used.calls} times in ${used.sessions.size} of ${sessions} sessions, ` +
      `${used.errors} errors, last ${used.last.slice(0, 10)}, suggest: port`
    );
  }
  if (tools.length === 0) return "no old tool uses it, suggest: not needed";
  const used = tools
    .map((tool) => ({ tool, calls: usage.counts.get(`tool:${tool}`)?.calls ?? 0 }))
    .filter(({ calls }) => calls > 0);
  if (used.length === 0)
    return `its tools (${tools.join(", ")}) were never used in ${sessions} sessions, suggest: not needed`;
  const through = used.map(({ tool, calls }) => `${tool} (${calls} calls)`).join(", ");
  return `used through ${through}, suggest: decide together with ${used.map(({ tool }) => tool).join(", ")}`;
}

// endregion: --- Usage

// region:    --- Ledger

export function parseLedger(text: string): Row[] {
  const rows: Row[] = [];
  let kind: Kind | undefined;
  for (const [index, line] of text.split("\n").entries()) {
    const heading = line.match(/^## (.+)$/);
    if (heading) kind = KINDS.find((entry) => entry.heading === heading[1].trim())?.kind;
    if (!line.startsWith("| `")) continue;
    const cells = line
      .split(/(?<!\\)\|/)
      .slice(1, -1)
      .map((cell) => cell.trim().replaceAll("\\|", "|"));
    const [name, status, detail, suggestion] = cells;
    if (!kind) throw new Error(`Line ${index + 1}: a row is not under a known kind heading.`);
    if (cells.length !== 4) throw new Error(`Line ${index + 1}: a row needs 4 cells.`);
    if (!STATUSES.includes(status as Status))
      throw new Error(`Line ${index + 1}: "${status}" is not a status. Use one of: ${STATUSES.join(", ")}.`);
    rows.push({ kind, name: name.replaceAll("`", ""), status: status as Status, detail, suggestion });
  }
  return rows;
}

export function renderLedger(rows: Row[], usage: Pick<Usage, "sessions" | "first" | "last">): string {
  const cell = (text: string) => text.replaceAll("|", "\\|");
  const lines = [
    "# Port ledger",
    "",
    "Every feature of the old enfusion-mcp project, and what happens to it in this repo. Run `/port-status` to update this file.",
    "",
    "- **Status**: `ported`, `not needed`, or `not yet`.",
    "- **Detail**: for `ported`, a link to the new Tool or skill. For `not needed`, the reason. For `not yet`, the port ticket, if one exists.",
    "- **Suggestion**: what the use of the old server in Claude Code session history suggests. Only the owner decides.",
    "",
    `History: ${usage.sessions.size} sessions with the old server, from ${usage.first.slice(0, 10) || "-"} to ${usage.last.slice(0, 10) || "-"}.`,
  ];
  for (const { kind, heading } of KINDS) {
    const ofKind = rows.filter((row) => row.kind === kind);
    if (ofKind.length === 0) continue;
    lines.push("", `## ${heading}`, "", "| Feature | Status | Detail | Suggestion |", "| --- | --- | --- | --- |");
    for (const row of ofKind)
      lines.push(`| \`${row.name}\` | ${row.status} | ${cell(row.detail)} | ${cell(row.suggestion)} |`);
  }
  return `${lines.join("\n")}\n`;
}

/** Adds the features that the ledger does not list yet, and gives every row a new suggestion. */
export function sync(rows: Row[], features: Feature[], usage: Usage) {
  const key = (item: { kind: Kind; name: string }) => `${item.kind}:${item.name}`;
  const known = new Map(features.map((feature) => [key(feature), feature]));
  const listed = new Set(rows.map(key));
  const added = features.filter((feature) => !listed.has(key(feature)));
  const stale = rows.filter((row) => !known.has(key(row)));
  const synced: Row[] = [
    ...rows,
    ...added.map((feature) => ({ kind: feature.kind, name: feature.name, status: "not yet" as const, detail: "", suggestion: "" })),
  ].map((row) => ({
    ...row,
    suggestion: known.has(key(row)) ? suggest(known.get(key(row))!, usage) : "not in the old project",
  }));
  const order = (row: Row) => KINDS.findIndex((entry) => entry.kind === row.kind);
  synced.sort((a, b) => order(a) - order(b) || a.name.localeCompare(b.name));
  return { rows: synced, added, stale };
}

/**
 * Gives every row that a selector names the same status and detail. A selector is a name,
 * a glob such as "*_create", or either of them after a kind, such as "template:prefab".
 */
export function setStatus(rows: Row[], selectors: string[], status: Status, detail: string) {
  if (status === "not needed" && !detail) throw new Error('"not needed" needs a reason.');
  if (status === "ported" && !detail) throw new Error('"ported" needs a link to the new Tool or skill.');
  const chosen = new Set<Row>();
  for (const selector of selectors) {
    const [kind, pattern] = selector.includes(":") ? selector.split(":", 2) : [undefined, selector];
    if (kind && !KINDS.some((entry) => entry.kind === kind))
      throw new Error(`"${kind}" is not a kind. Use one of: ${KINDS.map((entry) => entry.kind).join(", ")}.`);
    const glob = new RegExp(`^${pattern.replace(/[.+?^${}()[\]\\|]/g, "\\$&").replaceAll("*", ".*")}$`);
    const found = rows.filter((row) => (!kind || row.kind === kind) && glob.test(row.name));
    if (found.length === 0) throw new Error(`No feature matches "${selector}".`);
    const kinds = [...new Set(found.map((row) => row.kind))];
    if (kinds.length > 1)
      throw new Error(
        `"${selector}" matches features of more than one kind. Write ${kinds.map((k) => `${k}:${pattern}`).join(" or ")}.`,
      );
    for (const row of found) chosen.add(row);
  }
  const updated = rows.map((row) => (chosen.has(row) ? { ...row, status, detail } : row));
  return { rows: updated, changed: [...chosen] };
}

// endregion: --- Ledger

// region:    --- New repo

/** Makes the ledger link for a Tool or skill of this repo. It fails when the repo does not have it. */
export function linkTo(newRepo: string, kind: "tool" | "skill", name: string): string {
  if (kind === "skill") {
    const file = join("skills", name, "SKILL.md");
    if (!existsSync(join(newRepo, file))) throw new Error(`No skill named ${name}: ${file} does not exist.`);
    return `[skill ${name}](../${file})`;
  }
  const file = filesUnder(join(newRepo, "src"), ".rs").find((path) => declaresTool(path, name));
  if (!file) throw new Error(`No Tool named ${name} in src/.`);
  return `[tool ${name}](../${relative(newRepo, file)})`;
}

/** Finds the "ported" rows whose link does not lead to a Tool or skill in this repo. */
export function checkPorted(rows: Row[], newRepo: string): Problem[] {
  const problems: Problem[] = [];
  for (const row of rows.filter((row) => row.status === "ported")) {
    const link = row.detail.match(/^\[(tool|skill) ([^\]]+)\]\(\.\.\/([^)]+)\)$/);
    if (!link) {
      problems.push({ row, problem: "has no link to a Tool or skill" });
      continue;
    }
    const [, kind, name, file] = link;
    const path = join(newRepo, file);
    if (!existsSync(path)) problems.push({ row, problem: `links to ${file}, which does not exist` });
    else if (kind === "tool" && !declaresTool(path, name))
      problems.push({ row, problem: `links to Tool ${name}, but ${file} does not declare it` });
  }
  return problems;
}

function declaresTool(file: string, name: string): boolean {
  return new RegExp(`Tool::new\\(\\s*"${name}"`).test(readFileSync(file, "utf8"));
}

// endregion: --- New repo

/** The last line of a report: done, or the features that are still open. */
export function verdict(rows: Row[], problems: Problem[]): string {
  const broken = new Set(problems.map(({ row }) => row));
  const open = rows.filter((row) => row.status === "not yet" || broken.has(row));
  if (open.length === 0) return "Port done: nothing left in the old repo.";
  const lines = [`Not done: ${open.length} features still open.`];
  for (const { kind, heading } of KINDS) {
    const names = open.filter((row) => row.kind === kind).map((row) => row.name);
    if (names.length > 0) lines.push(`- ${heading}: ${names.join(", ")}`);
  }
  return lines.join("\n");
}

// region:    --- Support

function filesUnder(dir: string, extension: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir, { recursive: true, encoding: "utf8" })
    .map((path) => join(dir, path))
    .filter((path) => path.endsWith(extension) && statSync(path).isFile())
    .sort();
}

function namesUnder(dir: string, extension: string): string[] {
  return filesUnder(dir, extension).map((file) => withoutExtension(relative(dir, file)));
}

function withoutExtension(path: string): string {
  return path.replace(/\.[^./]+$/, "");
}

function matches(text: string, pattern: RegExp): string[] {
  return [...text.matchAll(pattern)].map((match) => match[1]);
}

// endregion: --- Support

// region:    --- Command line

const HELP = `Usage: node .claude/skills/port-status/port-status.ts <command>

  report
      Add missing features to docs/port-ledger.md, write new suggestions,
      check the "ported" rows, and print the open rows and the verdict.

  set <ported|not-needed|not-yet> <selector>... [--reason <text>] [--tool <name>] [--skill <name>] [--ticket <url>]
      Give every row that a selector names the same status.
      A selector is a name, a glob such as "*_create", or "<kind>:<name or glob>".
      not-needed needs --reason. ported needs --tool or --skill. not-yet takes an optional --ticket.

Settings (environment variables):
  PORT_STATUS_OLD_REPO   the old project (default ~/www/arma-enfusion-mcp)
  PORT_STATUS_HISTORY    Claude Code session history (default ~/.claude/projects)`;

function main(args: string[]): number {
  const newRepo = resolve(import.meta.dirname, "..", "..", "..");
  const ledgerPath = join(newRepo, "docs", "port-ledger.md");
  const oldRepo = process.env.PORT_STATUS_OLD_REPO ?? join(homedir(), "www", "arma-enfusion-mcp");
  const historyDir = process.env.PORT_STATUS_HISTORY ?? join(homedir(), ".claude", "projects");
  const [command, ...rest] = args;

  if (command === "report") {
    if (!existsSync(join(oldRepo, "src")))
      throw new Error(
        `The old project is not at ${oldRepo}. Clone git@github.com:lanka-community/arma-enfusion-mcp.git there.`,
      );
    const features = inventory(oldRepo);
    const usage = readUsage(historyDir, features);
    const before = existsSync(ledgerPath) ? parseLedger(readFileSync(ledgerPath, "utf8")) : [];
    const { rows, added, stale } = sync(before, features, usage);
    writeFileSync(ledgerPath, renderLedger(rows, usage));
    const problems = checkPorted(rows, newRepo);

    console.log(
      `History: ${usage.sessions.size} sessions with the old server, from ${usage.first.slice(0, 10) || "-"} to ${usage.last.slice(0, 10) || "-"}.`,
    );
    if (added.length > 0) console.log(`\nAdded to the ledger: ${added.map((f) => `${f.kind}:${f.name}`).join(", ")}`);
    if (stale.length > 0)
      console.log(`\nIn the ledger, but not in the old project: ${stale.map((r) => `${r.kind}:${r.name}`).join(", ")}`);
    if (problems.length > 0) {
      console.log('\n"Ported" rows that do not lead to a Tool or skill:');
      for (const { row, problem } of problems) console.log(`- ${row.kind}:${row.name} ${problem}`);
    }
    const open = rows.filter((row) => row.status === "not yet");
    if (open.length > 0) {
      console.log("\nNot yet decided:");
      for (const row of open)
        console.log(`- ${row.kind}:${row.name}: ${row.suggestion}${row.detail ? ` (ticket: ${row.detail})` : ""}`);
    }
    console.log(`\n${verdict(rows, problems)}`);
    return 0;
  }

  if (command === "set") {
    const [word, ...words] = rest;
    const status = ({ ported: "ported", "not-needed": "not needed", "not-yet": "not yet" } as const)[word];
    if (!status) throw new Error(`"${word}" is not a status. Use ported, not-needed, or not-yet.`);
    const options = new Map<string, string>();
    const selectors: string[] = [];
    for (let i = 0; i < words.length; i++) {
      if (words[i].startsWith("--")) options.set(words[i].slice(2), words[++i] ?? "");
      else selectors.push(words[i]);
    }
    if (selectors.length === 0) throw new Error("Name at least one feature.");
    let detail = "";
    if (status === "not needed") detail = options.get("reason") ?? "";
    if (status === "not yet") detail = options.get("ticket") ?? "";
    if (status === "ported") {
      if (options.has("tool")) detail = linkTo(newRepo, "tool", options.get("tool")!);
      if (options.has("skill")) detail = linkTo(newRepo, "skill", options.get("skill")!);
    }
    if (!existsSync(ledgerPath)) throw new Error("docs/port-ledger.md does not exist. Run the report command first.");
    const text = readFileSync(ledgerPath, "utf8");
    const { rows, changed } = setStatus(parseLedger(text), selectors, status, detail);
    // Keep the history line of the last report.
    const history = text.match(/^History: .*$/m)?.[0] ?? "";
    const rendered = renderLedger(rows, { sessions: new Set(), first: "", last: "" });
    writeFileSync(ledgerPath, rendered.replace(/^History: .*$/m, history));
    console.log(`Marked ${changed.length} rows as "${status}": ${changed.map((r) => `${r.kind}:${r.name}`).join(", ")}`);
    return 0;
  }

  console.log(HELP);
  return command === undefined || command === "help" ? 0 : 1;
}

if (import.meta.main) {
  try {
    process.exitCode = main(process.argv.slice(2));
  } catch (error) {
    console.error((error as Error).message);
    process.exitCode = 1;
  }
}
