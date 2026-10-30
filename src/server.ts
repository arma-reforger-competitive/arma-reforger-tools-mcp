import { McpServer } from "@modelcontextprotocol/server";
import { registerWikiTools } from "./wiki/tools.ts";

export function createServer(): McpServer {
  const server = new McpServer({ name: "reforger", version: "0.1.0" });
  registerWikiTools(server);
  return server;
}
