import assert from "node:assert/strict";
import { existsSync, mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { homedir, tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { type Feature, inventory, readUsage, type Usage } from "./old-project.ts";
import {
  checkPorted,
  linkTo,
  parseLedger,
  renderLedger,
  type Row,
  setStatus,
  sync,
  verdict,
} from "./port-status.ts";

const noUsage = (): Usage => ({ counts: new Map(), sessions: new Set(["s1", "s2", "s3", "s4"]), first: "", last: "" });
const row = (name: string, status: Row["status"] = "not yet", detail = "", kind: Row["kind"] = "tool"): Row => ({
  kind,
  name,
  status,
  detail,
  suggestion: "",
});

function tempDir(files: Record<string, string>): string {
  const dir = mkdtempSync(join(tmpdir(), "port-status-"));
  for (const [path, text] of Object.entries(files)) {
    mkdirSync(join(dir, path, ".."), { recursive: true });
    writeFileSync(join(dir, path), text);
  }
  return dir;
}

test("sync adds missing features as not yet, keeps decisions, and gives every row a suggestion", () => {
  const features: Feature[] = [
    { kind: "tool", name: "api_search" },
    { kind: "tool", name: "mod" },
    { kind: "pattern", name: "game-mode" },
  ];
  const { rows, added } = sync([row("mod", "not needed", "Claude writes it")], features, noUsage());
  assert.deepEqual(
    added.map((feature) => feature.name),
    ["api_search", "game-mode"],
  );
  assert.deepEqual(
    rows.map(({ name, status, detail, suggestion }) => [name, status, detail, suggestion]),
    [
      ["api_search", "not yet", "", "never used in 4 sessions, suggest: not needed"],
      ["mod", "not needed", "Claude writes it", "never used in 4 sessions, suggest: not needed"],
      ["game-mode", "not yet", "", "its tools (mod) were never used in 4 sessions, suggest: not needed"],
    ],
  );
});

test("a decision for many rows updates every named row and writes the reason", () => {
  const rows = [row("config_create"), row("layout_create"), row("script_create"), row("api_search")];
  const { rows: updated, changed } = setStatus(rows, ["*_create"], "not needed", "Claude writes these files itself");
  assert.equal(changed.length, 3);
  for (const name of ["config_create", "layout_create", "script_create"])
    assert.deepEqual(
      updated.find((r) => r.name === name),
      row(name, "not needed", "Claude writes these files itself"),
    );
  assert.deepEqual(updated.find((r) => r.name === "api_search"), row("api_search"));
});

test("a not needed row can go back to not yet", () => {
  const { rows } = setStatus([row("mod", "not needed", "Claude writes it")], ["mod"], "not yet", "");
  assert.deepEqual(rows, [row("mod")]);
});

test("set refuses unknown names, names of two kinds, and a not needed row without a reason", () => {
  const rows = [row("prefab"), row("prefab", "not yet", "", "template")];
  assert.throws(() => setStatus(rows, ["nothing"], "not yet", ""), /No feature matches "nothing"/);
  assert.throws(() => setStatus(rows, ["prefab"], "not yet", ""), /tool:prefab or template:prefab/);
  assert.throws(() => setStatus(rows, ["tool:prefab"], "not needed", ""), /needs a reason/);
  assert.equal(setStatus(rows, ["template:prefab"], "not needed", "x").changed[0].kind, "template");
});

test("a ported row whose Tool or skill is missing from the repo is reported", () => {
  const repo = tempDir({ "src/wiki/read.rs": 'let tool = Tool::new(\n    "wiki_read",' });
  assert.equal(linkTo(repo, "tool", "wiki_read"), "[tool wiki_read](../src/wiki/read.rs)");
  assert.throws(() => linkTo(repo, "tool", "api_search"), /No Tool named api_search/);
  const rows = [
    row("wiki_read", "ported", "[tool wiki_read](../src/wiki/read.rs)"),
    row("wiki_search", "ported", "[tool wiki_search](../src/wiki/read.rs)"),
    row("create-mod", "ported", "[skill create-mod](../skills/create-mod/SKILL.md)", "prompt"),
    row("mod", "ported", "see the new mod skill"),
  ];
  assert.deepEqual(
    checkPorted(rows, repo).map(({ row, problem }) => `${row.name} ${problem}`),
    [
      "wiki_search links to Tool wiki_search, but src/wiki/read.rs does not declare it",
      "create-mod links to skills/create-mod/SKILL.md, which does not exist",
      "mod has no link to a Tool or skill",
    ],
  );
});

test("the verdict says the port is done only when no row is open", () => {
  assert.equal(verdict([row("wiki_read", "ported", "link"), row("mod", "not needed", "x")], []), "Port done: nothing left in the old repo.");
  const broken = row("wiki_read", "ported", "link");
  assert.equal(
    verdict([broken, row("api_search"), row("game-mode", "not yet", "", "pattern")], [{ row: broken, problem: "" }]),
    "Not done: 3 features still open.\n- Tools: wiki_read, api_search\n- Patterns: game-mode",
  );
});

test("the ledger reads back what it wrote, also with | in a reason", () => {
  const rows = [
    row("wiki_read", "ported", "[tool wiki_read](../src/wiki/read.rs)"),
    row("mod", "not needed", "a | b"),
    row("ui/timer_hud", "not yet", "", "recipe"),
  ];
  const text = renderLedger(rows, { sessions: new Set(["s1"]), first: "2026-11-01T10:00:00Z", last: "2026-11-15T10:00:00Z" });
  assert.match(text, /History: 1 sessions with the old server, from 2026-11-01 to 2026-11-15\./);
  assert.deepEqual(parseLedger(text), rows);
  assert.throws(() => parseLedger(text.replace("| not needed |", "| maybe |")), /"maybe" is not a status/);
});

test("usage counts calls, errors, sessions, prompts, and resources once each", () => {
  const line = (record: object) => JSON.stringify(record);
  const call = (id: string, name: string, input = {}) => ({
    type: "assistant",
    sessionId: "s1",
    timestamp: "2026-11-10T08:00:00Z",
    message: { content: [{ type: "tool_use", id, name, input }] },
  });
  const result = (id: string, error: boolean) => ({
    type: "user",
    sessionId: "s1",
    message: { content: [{ type: "tool_result", tool_use_id: id, is_error: error }] },
  });
  const session = [
    call("t1", "mcp__enfusion-mcp__game_read"),
    result("t1", true),
    call("t2", "mcp__enfusion-mcp__game_read"),
    result("t2", false),
    call("t3", "ReadMcpResourceTool", { server: "enfusion-mcp", uri: "enfusion://class/SCR_Foo" }),
    { type: "user", uuid: "u1", sessionId: "s1", message: { content: "<command-name>/mcp__enfusion-mcp__create-mod</command-name>" } },
  ]
    .map(line)
    .join("\n");
  // A subagent file repeats call t1: it counts once.
  const history = tempDir({
    "p/s1.jsonl": session,
    "p/s1/subagents/a.jsonl": [call("t1", "mcp__enfusion-mcp__game_read"), result("t1", true)].map(line).join("\n"),
    "p/s2.jsonl": line({ type: "user", sessionId: "s2", timestamp: "2026-11-12T08:00:00Z", message: { content: "enfusion-mcp is on" } }),
  });
  const usage = readUsage(history, [{ kind: "resource", name: "class-info", uri: "enfusion://class/" }]);
  const counted = Object.fromEntries(
    [...usage.counts].map(([key, c]) => [key, [c.calls, c.errors, c.sessions.size, c.last]]),
  );
  assert.deepEqual(counted, {
    "tool:game_read": [2, 1, 1, "2026-11-10T08:00:00Z"],
    "resource:class-info": [1, 0, 1, "2026-11-10T08:00:00Z"],
    "prompt:create-mod": [1, 0, 1, ""],
  });
  assert.deepEqual([usage.sessions.size, usage.first, usage.last], [2, "2026-11-10T08:00:00Z", "2026-11-12T08:00:00Z"]);
});

const oldRepo = process.env.PORT_STATUS_OLD_REPO ?? join(homedir(), "www", "arma-enfusion-mcp");
test("the inventory of the old project has all 52 tools, the prompts, and the resources", { skip: !existsSync(oldRepo) && "no old project on disk" }, () => {
  const features = inventory(oldRepo);
  const names = (kind: string) => features.filter((feature) => feature.kind === kind).map((feature) => feature.name);
  assert.equal(names("tool").length, 52);
  assert.ok(names("tool").includes("wb_state") && names("tool").includes("wiki_read"));
  assert.deepEqual(names("prompt"), ["create-mod", "modify-mod"]);
  assert.deepEqual(names("resource"), ["class-info", "group-info", "pattern-info"]);
  assert.ok(names("template").includes("prefab") && !names("template").some((name) => name.includes("recipe")));
  assert.ok(names("recipe").includes("ui/timer_hud"));
  assert.deepEqual(names("animation"), ["animation-graph-code"]);
});
