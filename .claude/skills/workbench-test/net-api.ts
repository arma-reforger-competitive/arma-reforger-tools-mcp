// Sends one raw call to the Workbench NET API and prints the answer. SKILL.md in this folder says
// when to use it. The wire format is the same as in src/workbench/wire.rs.
import { readFileSync } from "node:fs";
import { connect } from "node:net";

/** The name that Workbench sees for this client. */
const CLIENT_ID = "reforger-test";
const DEFAULT_PORT = 5775;
const CONNECT_MS = 3_000;
// ReloadScripts answers when the compile ends. The bridge waits 120 seconds for it.
const ANSWER_MS = 130_000;

/** The bytes of one request: the protocol version 1, then the client ID, "JsonRPC", and the payload. */
export function request(payload: string): Buffer {
  const version = Buffer.alloc(4);
  version.writeInt32LE(1);
  return Buffer.concat([version, ...[CLIENT_ID, "JsonRPC", payload].map(pascalString)]);
}

/** The status and the payload of a whole answer. An error status can come with no payload. */
export function answer(bytes: Buffer): { status: string; payload: string } {
  const status = readString(bytes, 0);
  if (status.end === bytes.length) return { status: status.text, payload: "" };
  const payload = readString(bytes, status.end);
  if (payload.end !== bytes.length)
    throw new Error(`The answer has ${bytes.length - payload.end} bytes after the payload.`);
  return { status: status.text, payload: payload.text };
}

/** The gateway of the default route in the text of /proc/net/route. Under WSL, it is the Windows host. */
export function defaultGateway(routes: string): string | undefined {
  for (const line of routes.split("\n").slice(1)) {
    const [, destination, gateway] = line.trim().split(/\s+/);
    // Each address is 8 hex digits in little-endian order.
    if (destination !== "00000000" || !/^[0-9A-Fa-f]{8}$/.test(gateway ?? "")) continue;
    return (gateway.match(/../g) ?? []).reverse().map((byte) => Number.parseInt(byte, 16)).join(".");
  }
  return undefined;
}

// region:    --- Support

function pascalString(text: string): Buffer {
  const bytes = Buffer.from(text, "utf8");
  const length = Buffer.alloc(4);
  length.writeUInt32LE(bytes.length);
  return Buffer.concat([length, bytes]);
}

function readString(bytes: Buffer, start: number): { text: string; end: number } {
  if (start + 4 > bytes.length) throw new Error("The answer ends too early.");
  const end = start + 4 + bytes.readUInt32LE(start);
  if (end > bytes.length) throw new Error("The answer ends too early.");
  return { text: bytes.toString("utf8", start + 4, end), end };
}

function isWsl(): boolean {
  try {
    return readFileSync("/proc/version", "utf8").toLowerCase().includes("microsoft");
  } catch {
    return false;
  }
}

/** The host and port of the NET API, from the same settings as the Workbench bridge. */
function address(): { host: string; port: number } {
  const setting = (name: string) => process.env[name]?.trim() || undefined;
  const port = Number(setting("REFORGER_WORKBENCH_PORT") ?? DEFAULT_PORT);
  if (!Number.isInteger(port) || port < 1 || port > 65_535)
    throw new Error(`REFORGER_WORKBENCH_PORT is "${process.env.REFORGER_WORKBENCH_PORT}", which is not a port.`);
  let host = setting("REFORGER_WORKBENCH_HOST");
  if (!host && isWsl()) {
    host = defaultGateway(readFileSync("/proc/net/route", "utf8"));
    if (!host) throw new Error("/proc/net/route has no default route. Set REFORGER_WORKBENCH_HOST.");
  }
  return { host: host ?? "127.0.0.1", port };
}

function send(host: string, port: number, bytes: Buffer): Promise<Buffer> {
  return new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    const socket = connect({ host, port });
    const fail = (message: string) => {
      socket.destroy();
      reject(new Error(message));
    };
    // Under WSL, the Windows firewall can drop the connection to a closed port, so a time-out
    // here also means that nothing listens.
    const connectTimer = setTimeout(() => fail(`Nothing answers on ${host}:${port}. Workbench is not running.`), CONNECT_MS);
    let answerTimer: NodeJS.Timeout | undefined;
    socket.on("connect", () => {
      clearTimeout(connectTimer);
      answerTimer = setTimeout(() => fail(`No answer from ${host}:${port} in ${ANSWER_MS / 1000} seconds.`), ANSWER_MS);
      socket.end(bytes);
    });
    socket.on("data", (chunk) => chunks.push(chunk));
    socket.on("end", () => {
      clearTimeout(answerTimer);
      resolve(Buffer.concat(chunks));
    });
    socket.on("error", (error: NodeJS.ErrnoException) => {
      clearTimeout(connectTimer);
      clearTimeout(answerTimer);
      if (error.code === "ECONNREFUSED") reject(new Error(`Nothing answers on ${host}:${port}. Workbench is not running.`));
      else reject(error);
    });
  });
}

// endregion: --- Support

// region:    --- Command line

const HELP = `Usage: node .claude/skills/workbench-test/net-api.ts <APIFunc> ['<JSON object with the parameters>']

  Sends one call to the Workbench NET API and prints the status line and the JSON payload.

Examples:
  node .claude/skills/workbench-test/net-api.ts IsWorkbenchRunning
  node .claude/skills/workbench-test/net-api.ts ReforgerBridge '{"action":"ping"}'

Settings (environment variables, the same as the Workbench bridge):
  REFORGER_WORKBENCH_HOST   the computer that runs Workbench (default: 127.0.0.1, under WSL the Windows host)
  REFORGER_WORKBENCH_PORT   the NET API port (default: 5775)`;

async function main(args: string[]): Promise<number> {
  const [func, params = "{}"] = args;
  if (!func || func === "help" || func === "--help") {
    console.log(HELP);
    return func ? 0 : 1;
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(params);
  } catch {
    parsed = undefined;
  }
  if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed))
    throw new Error(`The parameters must be a JSON object, for example '{"action":"ping"}'. Got: ${params}`);
  const { host, port } = address();
  const bytes = await send(host, port, request(JSON.stringify({ ...parsed, APIFunc: func })));
  const { status, payload } = answer(bytes);
  console.log(`status: ${status}`);
  if (payload) console.log(payload);
  return status === "Ok" ? 0 : 1;
}

if (import.meta.main) {
  try {
    process.exitCode = await main(process.argv.slice(2));
  } catch (error) {
    console.error((error as Error).message);
    process.exitCode = 1;
  }
}

// endregion: --- Command line
