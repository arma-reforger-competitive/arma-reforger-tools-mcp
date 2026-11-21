// Reads the features of the old project and their use in Claude Code session history.
import { existsSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";

import { filesUnder, matches, namesUnder, withoutExtension } from "./files.ts";

export type Kind =
  | "tool"
  | "prompt"
  | "resource"
  | "template"
  | "recipe"
  | "pattern"
  | "knowledge"
  | "animation";
/** One feature of the old project. `uri` is the start of a resource's URI, for example "enfusion://class/". */
export type Feature = { kind: Kind; name: string; uri?: string };
export type Count = { calls: number; errors: number; sessions: Set<string>; last: string };
/** Use of the old server in Claude Code session history. Keys of `counts` are "<kind>:<name>". */
export type Usage = { counts: Map<string, Count>; sessions: Set<string>; first: string; last: string };

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
