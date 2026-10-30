# wiki

Reads pages of the Arma Reforger wiki from the Knowledge base.

## Tools

- `wiki_read`: returns the full text of one page by its title, for example `Arma Reforger:Mod Project Setup`. Capital letters do not matter, and underscores from a wiki URL count as spaces. If no page has the title, the answer lists the 3 closest real titles. Pages longer than 100,000 characters are cut.

## Data

`pages.json` holds 258 pages:

- 251 pages from the [BI Community Wiki](https://community.bistudio.com/wiki/Category:Arma_Reforger/Modding), the Arma Reforger modding category.
- 7 pages from the Enfusion engine docs that ship with the Workbench. These pages have no URL.

The data comes from the old enfusion-mcp project, which scraped it with `scripts/scrape-wiki.ts`. This repo has no scraper yet.
