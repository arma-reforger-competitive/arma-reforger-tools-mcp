// Calls one Tool of target/release/reforger through MCP Inspector and prints only its answer.
// SKILL.md in this folder says when to use it.
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";

const INSPECTOR = "@modelcontextprotocol/inspector@2.15.0";
// MCP Inspector stops a call after 60 seconds. workbench_launch can wait 180 seconds, plus the
// Steam start.
const TIMEOUT_MS = "300000";

/** The `REFORGER_*` settings: the `env` of Claude Code's settings.json, then the shell, which wins. */
export function settings(settingsEnv: Record<string, string>, shellEnv: NodeJS.ProcessEnv): Record<string, string> {
  const all: Record<string, string> = {};
  for (const source of [settingsEnv, shellEnv])
    for (const [key, value] of Object.entries(source))
      if (key.startsWith("REFORGER_") && value !== undefined) all[key] = value;
  return all;
}

/** The arguments of `npx` for one Tool call with the settings and the `name=value` inputs. */
export function inspectorArgs(env: Record<string, string>, server: string, tool: string, inputs: string[]): string[] {
  return [
    "-y",
    INSPECTOR,
    "--cli",
    ...Object.entries(env).flatMap(([key, value]) => ["-e", `${key}=${value}`]),
    server,
    "--method",
    "tools/call",
    "--tool-name",
    tool,
    ...inputs.flatMap((input) => ["--tool-arg", input]),
  ];
}

/** The text of the answer, and whether it is an error. */
export function answerText(stdout: string): { text: string; isError: boolean } {
  const answer = JSON.parse(stdout);
  if (answer.error) return { text: `MCP Inspector: ${answer.error.message}`, isError: true };
  const text = (answer.content ?? []).map((block: { text?: string }) => block.text ?? "").join("\n");
  return { text, isError: answer.isError === true };
}

function main(): void {
  const [tool, ...inputs] = process.argv.slice(2);
  if (!tool) {
    console.error("Usage: node .claude/skills/workbench-test/tool.ts <tool> [name=value ...]");
    process.exit(2);
  }
  const settingsFile = JSON.parse(readFileSync(join(homedir(), ".claude/settings.json"), "utf8"));
  const env = settings(settingsFile.env ?? {}, process.env);
  const server = join(import.meta.dirname, "../../../target/release/reforger");
  let stdout: string;
  try {
    stdout = execFileSync("npx", inspectorArgs(env, server, tool, inputs), {
      encoding: "utf8",
      env: { ...process.env, MCP_SERVER_REQUEST_TIMEOUT: TIMEOUT_MS },
      stdio: ["ignore", "pipe", "ignore"],
    });
  } catch (error) {
    // MCP Inspector exits with 1 on a timeout, and still prints its JSON answer.
    stdout = (error as { stdout?: string }).stdout ?? "";
    if (!stdout.trim()) throw error;
  }
  const { text, isError } = answerText(stdout);
  console.log(text);
  if (isError) process.exit(1);
}

if (import.meta.main) main();
