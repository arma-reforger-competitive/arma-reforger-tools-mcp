# Rebuild by porting

We forked enfusion-mcp (52 tools, about 19,600 lines) but wrote only 10% of it, and we used only 8 of its tools. We start a new, empty repo and move one Capability at a time from the old code, only when a real task needs it. We rewrite each part on the way and test it by hand. A full rewrite would throw away hard, already-solved parts (the Workbench protocol, the .pak reader, the Knowledge base). Cleaning the old repo in place would keep tools that nobody chose.

The old code is MIT-licensed. Copied code keeps its MIT notice ("Copyright (c) 2025 enfusion-mcp contributors").
