import type { McpServer } from "@modelcontextprotocol/server";
import * as z from "zod";
import { closestTitles, findPage, type WikiPage } from "./pages.ts";

const SOURCE_NAMES: Record<WikiPage["source"], string> = {
  "bistudio-wiki": "BI Community Wiki",
  enfusion: "Enfusion engine docs",
};

// Claude Code cuts tool answers longer than about 25,000 tokens, which is about 100,000 characters.
const MAX_CONTENT_LENGTH = 100_000;

function pageContent(content: string): string {
  if (content.length <= MAX_CONTENT_LENGTH) return content;
  const count = (n: number) => n.toLocaleString("en-US");
  return `${content.slice(0, MAX_CONTENT_LENGTH)}\n\n(Cut at ${count(MAX_CONTENT_LENGTH)} of ${count(content.length)} characters.)`;
}

export function registerWikiTools(server: McpServer): void {
  server.registerTool(
    "wiki_read",
    {
      description:
        "Read the full text of one Arma Reforger wiki page by its title. " +
        "If no page has that title, the answer lists the closest real titles.",
      inputSchema: z.object({
        title: z.string().min(1).describe('Title of the wiki page, for example "Arma Reforger:Mod Project Setup"'),
      }),
    },
    async ({ title }) => {
      const page = findPage(title);
      if (!page) {
        const suggestions = closestTitles(title, 3).map((suggestion) => `"${suggestion}"`).join(", ");
        const text = `No wiki page has the title "${title}". The closest titles are: ${suggestions}.`;
        return { content: [{ type: "text", text }], isError: true };
      }
      const source = page.url ? `${SOURCE_NAMES[page.source]}, ${page.url}` : SOURCE_NAMES[page.source];
      const text = [`## ${page.title}`, `Source: ${source}`, "", pageContent(page.content)].join("\n");
      return { content: [{ type: "text", text }] };
    },
  );
}
