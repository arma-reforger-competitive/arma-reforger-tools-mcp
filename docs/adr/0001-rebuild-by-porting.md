# Rebuild by porting

We forked enfusion-mcp (52 tools, about 19,600 lines) but wrote only 10% of it, and we used only 8 of its tools. We start a new, empty repo and move one Capability at a time from the old code, only when a real task needs it. Each Port translates the old TypeScript into Rust (ADR 0008): it keeps the solved logic and the old expected answers, and fixes what is not sound. Starting from nothing would throw away hard, already-solved parts (the Workbench protocol, the .pak reader, the Knowledge base). Cleaning the old repo in place would keep tools that nobody chose.

The old code is MIT-licensed. Code translated from it keeps its MIT notice ("Copyright (c) 2025 enfusion-mcp contributors").
