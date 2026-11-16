//! The `wiki_search` Tool: finds pages by words with a BM25 ranking over Sections (ADR 0005).

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;

use rmcp::model::{CallToolResult, ContentBlock};
use schemars::JsonSchema;
use serde::Deserialize;

use super::pages::{Page, Pages};
use super::sections::Section;

/// BM25 settings: how fast more uses of a word stop adding to the score, and how much a long text is
/// pushed down. These are the usual values.
const K1: f64 = 1.2;
const B: f64 = 0.75;

const DEFAULT_LIMIT: usize = 5;
const MAX_LIMIT: usize = 10;
const SNIPPET_CHARS: usize = 200;

#[derive(Deserialize, JsonSchema)]
pub(super) struct SearchInput {
    /// Words to search for, for example "server config" or "faction spawn points"
    #[schemars(length(min = 1))]
    query: String,

    /// How many pages to return, from 1 to 10, for example 3. The default is 5.
    #[schemars(range(min = 1, max = 10))]
    limit: Option<usize>,
}

/// The words of every page title and every Section, ready for search.
///
/// A page scores its title, plus its best Section. The title counts once per page, so the best
/// Section of a page is the one whose own text matches best.
pub(super) struct Index {
    titles: Bm25,
    sections: Bm25,

    /// For each entry of `sections`: where its Section is.
    places: Vec<Place>,
}

/// Answers one `wiki_search` call.
pub(super) fn search(pages: &Pages, index: &Index, input: &SearchInput) -> CallToolResult {
    let query = input.query.trim();
    let query_words: HashSet<String> = words(query).into_iter().collect();
    if query_words.is_empty() {
        let text =
            "The query has no words. Give words to search for, for example \"server config\".";
        return CallToolResult::error(vec![ContentBlock::text(text)]);
    }
    let limit = input.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let hits = index.search(&query_words, limit);
    if hits.is_empty() {
        let text = format!(
            "No wiki page matches \"{query}\". Try fewer words, another word for the same thing, \
             or the name of a class or a setting, for example \"replication\" or \"fastValidation\"."
        );
        return CallToolResult::success(vec![ContentBlock::text(text)]);
    }
    let mut text = format!(
        "{} pages match \"{query}\". Read one with wiki_read: give its title, and give `section` \
         to read only that Section.\n",
        hits.len()
    );
    for (number, hit) in hits.iter().enumerate() {
        let Some(page) = pages.all().get(hit.place.page) else {
            continue;
        };
        let Some(section) = page.sections.get(hit.place.section) else {
            continue;
        };
        let path = if section.path.is_empty() {
            "(the text before the first heading)".to_owned()
        } else {
            section.path_text()
        };
        let _ = write!(
            text,
            "\n{}. {}\n   Section: {path}\n   {}\n",
            number + 1,
            page.title,
            snippet(page, section, &query_words)
        );
    }
    CallToolResult::success(vec![ContentBlock::text(text)])
}

impl Index {
    /// Indexes each title, and each Section with its heading path and its own text. A heading with no
    /// text of its own only groups smaller headings, and their paths hold its name, so it is left out.
    pub(super) fn new(pages: &Pages) -> Self {
        let mut titles = Vec::new();
        let mut sections = Vec::new();
        let mut places = Vec::new();
        for (page_index, page) in pages.all().iter().enumerate() {
            titles.push(words(&page.title));
            for (section_index, section) in page.sections.iter().enumerate() {
                if text_below_heading(page, section).trim().is_empty() {
                    continue;
                }
                let text = format!("{} {}", section.path_text(), searchable_text(page, section));
                sections.push(words(&text));
                places.push(Place {
                    page: page_index,
                    section: section_index,
                });
            }
        }
        Self {
            titles: Bm25::new(&titles),
            sections: Bm25::new(&sections),
            places,
        }
    }

    /// Gives the best Section of each page that matches, the best page first. A page that matches only
    /// by its title gives its first Section.
    fn search(&self, query: &HashSet<String>, limit: usize) -> Vec<Hit> {
        let mut best: HashMap<usize, Hit> = HashMap::new();
        for (entry, score) in self.sections.scores(query) {
            let Some(&place) = self.places.get(entry) else {
                continue;
            };
            if best.get(&place.page).is_none_or(|hit| score > hit.score) {
                best.insert(place.page, Hit { place, score });
            }
        }
        for (page, title_score) in self.titles.scores(query) {
            let first_section = self
                .places
                .iter()
                .find(|place| place.page == page)
                .map_or(0, |place| place.section);
            let hit = best.entry(page).or_insert(Hit {
                place: Place {
                    page,
                    section: first_section,
                },
                score: 0.0,
            });
            hit.score += title_score;
        }
        let mut hits: Vec<Hit> = best.into_values().collect();
        hits.sort_by(|a, b| {
            b.score
                .total_cmp(&a.score)
                .then(a.place.page.cmp(&b.place.page))
        });
        hits.truncate(limit);
        hits
    }
}

// region:    --- Support

/// A Section, as positions in `Pages::all` and `Page::sections`.
#[derive(Clone, Copy)]
struct Place {
    page: usize,
    section: usize,
}

struct Hit {
    place: Place,
    score: f64,
}

/// A BM25 ranking over a list of texts, each given as its words.
struct Bm25 {
    texts: Vec<Counts>,

    /// For each word: in how many texts it is.
    texts_with: HashMap<String, u32>,
    average_len: f64,
}

struct Counts {
    counts: HashMap<String, u32>,

    /// The number of words.
    len: u32,
}

impl Bm25 {
    fn new(texts: &[Vec<String>]) -> Self {
        let mut texts_with: HashMap<String, u32> = HashMap::new();
        let texts: Vec<Counts> = texts
            .iter()
            .map(|words| {
                let mut counts: HashMap<String, u32> = HashMap::new();
                for word in words {
                    *counts.entry(word.clone()).or_insert(0) += 1;
                }
                for word in counts.keys() {
                    *texts_with.entry(word.clone()).or_insert(0) += 1;
                }
                Counts {
                    counts,
                    len: to_u32(words.len()),
                }
            })
            .collect();
        let total_len: f64 = texts.iter().map(|text| f64::from(text.len)).sum();
        let average_len = total_len / f64::from(to_u32(texts.len()).max(1));
        Self {
            texts,
            texts_with,
            average_len,
        }
    }

    /// Gives the position and the score of each text that has a word of the query.
    fn scores(&self, query: &HashSet<String>) -> Vec<(usize, f64)> {
        let total = f64::from(to_u32(self.texts.len()));
        let weights: Vec<(&String, f64)> = query
            .iter()
            .filter_map(|word| {
                let with = f64::from(*self.texts_with.get(word)?);
                Some((word, ((total - with + 0.5) / (with + 0.5) + 1.0).ln()))
            })
            .collect();
        self.texts
            .iter()
            .enumerate()
            .map(|(index, text)| {
                let len_factor = 1.0 - B + B * f64::from(text.len) / self.average_len;
                let score: f64 = weights
                    .iter()
                    .map(|(word, weight)| {
                        let count = f64::from(text.counts.get(*word).copied().unwrap_or(0));
                        weight * count * (K1 + 1.0) / (count + K1 * len_factor)
                    })
                    .sum();
                (index, score)
            })
            .filter(|(_, score)| *score > 0.0)
            .collect()
    }
}

/// The Knowledge base has far fewer than 4 billion words, so a count fits in `u32`.
fn to_u32(len: usize) -> u32 {
    u32::try_from(len).unwrap_or(u32::MAX)
}

/// Splits text into lowercase words. A code name stays one word, and its parts are words too:
/// `missionHeader` gives `missionheader`, `mission`, and `header`. One-letter words are left out.
fn words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    for name in text.split(|c: char| !(c.is_alphanumeric() || c == '_')) {
        let parts = name_parts(name);
        if parts.len() > 1 {
            words.extend(parts.into_iter().filter(|part| part.chars().count() > 1));
        }
        if name.chars().count() > 1 {
            words.push(name.to_lowercase());
        }
    }
    words
}

/// Splits a code name at `_` and where a capital letter follows a small letter or a digit:
/// `SCR_BaseGameMode` gives `scr`, `base`, `game`, `mode`.
fn name_parts(name: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut part = String::new();
    let mut previous: Option<char> = None;
    for c in name.chars() {
        let new_part = c == '_'
            || (c.is_uppercase()
                && previous.is_some_and(|p| p.is_lowercase() || p.is_ascii_digit()));
        if new_part && !part.is_empty() {
            parts.push(std::mem::take(&mut part));
        }
        if c != '_' {
            part.extend(c.to_lowercase());
        }
        previous = Some(c);
    }
    if !part.is_empty() {
        parts.push(part);
    }
    parts
}

/// The own text of a Section, with the heading line, without the URLs of Markdown links.
fn searchable_text(page: &Page, section: &Section) -> String {
    without_link_urls(section.own_text(page.content))
}

/// The own text of a Section without its heading line.
fn text_below_heading<'a>(page: &'a Page, section: &Section) -> &'a str {
    let text = section.own_text(page.content);
    if section.path.is_empty() {
        return text;
    }
    text.split_once('\n').map_or("", |(_, rest)| rest)
}

/// `[words](https://...)` keeps only `words`: every link URL has the same few words.
fn without_link_urls(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("](") {
        out.push_str(rest.get(..=start).unwrap_or_default());
        let after = rest.get(start + 2..).unwrap_or_default();
        rest = after
            .find([')', '\n'])
            .and_then(|end| after.get(end + 1..))
            .unwrap_or_default();
    }
    out.push_str(rest);
    out.replace(['[', ']'], "")
}

/// The first line of the Section that has a word of the query, cut to about 200 characters.
fn snippet(page: &Page, section: &Section, query: &HashSet<String>) -> String {
    let text = searchable_text(page, section);
    let lines: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with("```"))
        .collect();
    let line = lines
        .iter()
        .find(|line| words(line).iter().any(|word| query.contains(word)))
        .or_else(|| lines.first())
        .copied()
        .unwrap_or_default();
    let line = line.split_whitespace().collect::<Vec<_>>().join(" ");
    if line.chars().count() <= SNIPPET_CHARS {
        return line;
    }
    let cut: String = line.chars().take(SNIPPET_CHARS).collect();
    format!("{}…", cut.trim_end())
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_support::{self, Answer, TestClient};

    #[tokio::test]
    async fn finds_server_config_near_the_top_for_server_config() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "server config" })).await;

        // -- Check
        assert!(!answer.is_error);
        assert!(
            titles(&answer)
                .iter()
                .take(3)
                .any(|title| title == "Arma Reforger:Server Config")
        );
    }

    #[tokio::test]
    async fn finds_the_expected_page_in_the_top_3_for_real_searches() {
        // -- Setup & Fixtures
        // Searches that Claude sent to the old server, with the page that answers them in the wiki.
        let cases = [
            ("server config", "Arma Reforger:Server Config"),
            (
                "addonDownloadDir startup parameter",
                "Arma Reforger:Startup Parameters",
            ),
            (
                "Input Manager contexts action priority InputSource filter",
                "Arma Reforger:Input Manager",
            ),
            ("faction spawn points", "Arma Reforger:Faction Creation"),
            (
                "mission header faction player limit",
                "Arma Reforger:Server Config",
            ),
            (
                "Workbench add dependency workshop mod",
                "Arma Reforger:Mod Project Setup",
            ),
            (
                "game mode states pregame waiting for players",
                "Arma Reforger:General Game Mode Setup",
            ),
            (
                "Game Master camera free roam editor",
                "Arma Reforger:Game Master",
            ),
            (
                "radio transceiver frequency range",
                "Arma Reforger:Audio: Voice over Network",
            ),
            ("replication", "Replication"),
            ("RplNode", "Replication: RplNode"),
        ];
        let client = connect().await;

        // -- Exec
        let mut misses = Vec::new();
        for (query, expected) in cases {
            let answer = search(&client, json!({ "query": query })).await;
            let top = titles(&answer);
            if !top.iter().take(3).any(|title| title == expected) {
                misses.push(format!("{query}: expected {expected}, got {top:?}"));
            }
        }

        // -- Check
        assert!(misses.is_empty(), "{misses:#?}");
    }

    #[tokio::test]
    async fn shows_each_page_once_with_the_path_of_its_best_section_and_a_snippet() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(
            &client,
            json!({ "query": "server config mods modID", "limit": 10 }),
        )
        .await;

        // -- Check
        let titles = titles(&answer);
        let unique: std::collections::HashSet<&String> = titles.iter().collect();
        assert_eq!(unique.len(), titles.len());
        assert!(answer.text.contains(
            "Arma Reforger:Server Config\n   Section: game > mods > modID\n   GUID of the mod."
        ));
    }

    #[tokio::test]
    async fn keeps_to_the_limit() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "game", "limit": 3 })).await;

        // -- Check
        assert_eq!(titles(&answer).len(), 3);
    }

    #[tokio::test]
    async fn says_what_to_try_next_when_no_page_matches() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "xyzzy99999qqq" })).await;

        // -- Check
        assert!(!answer.is_error);
        assert!(
            answer
                .text
                .starts_with("No wiki page matches \"xyzzy99999qqq\". Try fewer words")
        );
    }

    #[tokio::test]
    async fn matches_whole_words_only_not_parts_of_words() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "alidation" })).await;

        // -- Check
        assert!(answer.text.starts_with("No wiki page matches"));
    }

    #[tokio::test]
    async fn asks_for_words_when_the_query_has_none() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": " - " })).await;

        // -- Check
        assert!(answer.is_error);
        assert!(answer.text.contains("for example \"server config\""));
    }

    // -- Test Support

    async fn connect() -> TestClient {
        test_support::connect(crate::wiki::tools().unwrap()).await
    }

    async fn search(client: &TestClient, arguments: serde_json::Value) -> Answer {
        test_support::call(client, "wiki_search", arguments).await
    }

    /// The page titles in the answer, in order: the lines like "1. Arma Reforger:Server Config".
    fn titles(answer: &Answer) -> Vec<String> {
        answer
            .text
            .lines()
            .filter_map(|line| line.split_once(". "))
            .filter(|(number, _)| number.parse::<usize>().is_ok())
            .map(|(_, title)| title.to_owned())
            .collect()
    }
}

// endregion: --- Tests
