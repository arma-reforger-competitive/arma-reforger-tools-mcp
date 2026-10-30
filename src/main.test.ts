import { Client } from "@modelcontextprotocol/client";
import { StdioClientTransport } from "@modelcontextprotocol/client/stdio";
import { fileURLToPath } from "node:url";
import { expect, it } from "vitest";

it("starts the reforger server over stdio and answers wiki_read", async () => {
  const transport = new StdioClientTransport({
    command: process.execPath,
    args: [fileURLToPath(new URL("main.ts", import.meta.url))],
  });
  const client = new Client({ name: "test", version: "0" });
  await client.connect(transport);

  try {
    expect(client.getServerVersion()?.name).toBe("reforger");
    const result = await client.callTool({
      name: "wiki_read",
      arguments: { title: "Arma Reforger:Mod Project Setup" },
    });
    const [first] = result.content as { type: string; text: string }[];
    expect(first!.text).toContain("## Arma Reforger:Mod Project Setup");
  } finally {
    await client.close();
  }
});
