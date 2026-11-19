import assert from "node:assert/strict";
import { test } from "node:test";
import { answerText, inspectorArgs, settings } from "./tool.ts";

test("the settings are the REFORGER_ values of settings.json, and the shell wins", () => {
  const fromFile = { REFORGER_GAME_PATH: "/mnt/d/Game", REFORGER_MODS_PATH: "D:\\Mods", OTHER: "x" };
  const fromShell = { REFORGER_MODS_PATH: "/tmp/test-lists", PATH: "/usr/bin" };
  assert.deepEqual(settings(fromFile, fromShell), {
    REFORGER_GAME_PATH: "/mnt/d/Game",
    REFORGER_MODS_PATH: "/tmp/test-lists",
  });
});

test("the npx arguments pass each setting and each input", () => {
  assert.deepEqual(inspectorArgs({ REFORGER_GAME_PATH: "/g" }, "/r/reforger", "game_find", ["query=S3C"]), [
    "-y",
    "@modelcontextprotocol/inspector@2.15.0",
    "--cli",
    "-e",
    "REFORGER_GAME_PATH=/g",
    "/r/reforger",
    "--method",
    "tools/call",
    "--tool-name",
    "game_find",
    "--tool-arg",
    "query=S3C",
  ]);
});

test("the answer is the text of the Tool, or the error of MCP Inspector", () => {
  // The shapes that MCP Inspector 2.15.0 printed in the check of #77.
  const ok = '{"content":[{"type":"text","text":"S3C_CameraFix (6A09759015FFDA26): present"}],"isError":false}';
  assert.deepEqual(answerText(ok), { text: "S3C_CameraFix (6A09759015FFDA26): present", isError: false });
  const timeout = '{"error":{"code":"error","message":"Request timed out after 1m00s (tools/call).","cause":"x"}}';
  assert.deepEqual(answerText(timeout), {
    text: "MCP Inspector: Request timed out after 1m00s (tools/call).",
    isError: true,
  });
});
