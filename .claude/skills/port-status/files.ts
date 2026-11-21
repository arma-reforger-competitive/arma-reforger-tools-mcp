// Small helpers for files and text, for port-status.ts and old-project.ts.
import { existsSync, readdirSync, statSync } from "node:fs";
import { join, relative } from "node:path";

export function filesUnder(dir: string, extension: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir, { recursive: true, encoding: "utf8" })
    .map((path) => join(dir, path))
    .filter((path) => path.endsWith(extension) && statSync(path).isFile())
    .sort();
}

export function namesUnder(dir: string, extension: string): string[] {
  return filesUnder(dir, extension).map((file) => withoutExtension(relative(dir, file)));
}

export function withoutExtension(path: string): string {
  return path.replace(/\.[^./]+$/, "");
}

export function matches(text: string, pattern: RegExp): string[] {
  return [...text.matchAll(pattern)].map((match) => match[1]);
}
