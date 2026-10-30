import { Client, InMemoryTransport } from "@modelcontextprotocol/client";
import { beforeAll, describe, expect, it } from "vite-plus/test";
import { createServer } from "../server.ts";

let client: Client;

beforeAll(async () => {
  const [clientTransport, serverTransport] = InMemoryTransport.createLinkedPair();
  await createServer().connect(serverTransport);
  client = new Client({ name: "test", version: "0" });
  await client.connect(clientTransport);
});

async function readPage(title: string) {
  const result = await client.callTool({ name: "wiki_read", arguments: { title } });
  const [first] = result.content as { type: string; text: string }[];
  return { text: first!.text, isError: result.isError ?? false };
}

describe("wiki_read", () => {
  it("returns the page text for a known title", async () => {
    const page = await readPage("Arma Reforger:Mod Project Setup");

    expect(page.isError).toBe(false);
    expect(page.text).toContain("## Arma Reforger:Mod Project Setup");
    expect(page.text).toContain(
      "https://community.bistudio.com/wiki/Arma_Reforger:Mod_Project_Setup",
    );
    expect(page.text).toContain("Workbench Launcher Setup");
  });

  it("finds the page when the title has wrong capital letters or underscores from the URL", async () => {
    for (const title of ["arma reforger:mod project setup", "Arma_Reforger:Mod_Project_Setup"]) {
      const page = await readPage(title);

      expect(page.isError).toBe(false);
      expect(page.text).toContain("## Arma Reforger:Mod Project Setup");
    }
  });

  it("names the source of a page that has no URL", async () => {
    const page = await readPage("Event System");

    expect(page.isError).toBe(false);
    expect(page.text).toContain("Source: Enfusion engine docs\n");
    expect(page.text).not.toContain("undefined");
  });

  it("cuts a very long page at 100,000 characters and says so", async () => {
    const page = await readPage("Arma Reforger:Diag Menu");

    expect(page.isError).toBe(false);
    expect(page.text.length).toBeLessThan(100_500);
    expect(page.text).toContain("Cut at 100,000 of 164,192 characters.");
  });

  it("suggests the closest real titles when the title is unknown", async () => {
    for (const title of ["Mod Project Setup", "Arma Reforger:Mod Projct Setp"]) {
      const page = await readPage(title);

      expect(page.isError).toBe(true);
      expect(page.text).toContain(`No wiki page has the title "${title}"`);
      expect(page.text).toContain('"Arma Reforger:Mod Project Setup"');
    }
  });
});
