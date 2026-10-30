import { readFileSync } from "node:fs";

export interface WikiPage {
  title: string;
  source: "bistudio-wiki" | "enfusion";
  url?: string;
  content: string;
}

const pages: WikiPage[] = JSON.parse(readFileSync(new URL("pages.json", import.meta.url), "utf8"));

// Wiki URLs write spaces as underscores, so "Mod_Project_Setup" and "mod project setup" are the same title.
function titleKey(title: string): string {
  return title.replace(/_/g, " ").replace(/\s+/g, " ").trim().toLowerCase();
}

const pageByTitle = new Map(pages.map((page) => [titleKey(page.title), page]));

export function findPage(title: string): WikiPage | undefined {
  return pageByTitle.get(titleKey(title));
}

// Counts each pair of letters that stand next to each other: "setup" has "se", "et", "tu", "up".
function bigrams(text: string): Map<string, number> {
  const counts = new Map<string, number>();
  for (let i = 0; i < text.length - 1; i++) {
    const pair = text.slice(i, i + 2);
    counts.set(pair, (counts.get(pair) ?? 0) + 1);
  }
  return counts;
}

// Gives 1 when both texts have the same letter pairs and 0 when they share none (the Dice coefficient).
function similarity(a: string, b: string): number {
  const pairsA = bigrams(a);
  const pairsB = bigrams(b);
  let shared = 0;
  for (const [pair, count] of pairsA) shared += Math.min(count, pairsB.get(pair) ?? 0);
  return (2 * shared) / (a.length - 1 + (b.length - 1));
}

// Claude often leaves out the "Arma Reforger:" namespace, so a title is also compared without it.
function titleScore(query: string, page: WikiPage): number {
  const key = titleKey(page.title);
  const withoutNamespace = key.slice(key.indexOf(":") + 1);
  return Math.max(similarity(query, key), similarity(query, withoutNamespace));
}

export function closestTitles(title: string, limit: number): string[] {
  const query = titleKey(title);
  return pages
    .map((page) => ({ title: page.title, score: titleScore(query, page) }))
    .sort((a, b) => b.score - a.score)
    .slice(0, limit)
    .map((match) => match.title);
}
