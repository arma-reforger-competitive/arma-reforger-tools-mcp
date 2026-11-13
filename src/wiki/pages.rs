use std::collections::HashMap;

use serde::Deserialize;

// region:    --- Types

pub(super) struct Pages {
    pages: Vec<Page>,
    by_title: HashMap<String, usize>,
}

/// Every page from the BI Community Wiki has a URL. The Enfusion engine docs have none.
#[derive(Deserialize)]
#[serde(tag = "source", rename_all = "kebab-case")]
pub(super) enum Source {
    BistudioWiki { url: String },
    Enfusion,
}

#[derive(Deserialize)]
pub(super) struct Page {
    pub(super) title: String,

    #[serde(flatten)]
    pub(super) source: Source,
    pub(super) content: String,
}

// endregion: --- Types

impl Pages {
    pub(super) fn parse(json: &str) -> Result<Self, serde_json::Error> {
        let pages: Vec<Page> = serde_json::from_str(json)?;
        let by_title = pages
            .iter()
            .enumerate()
            .map(|(index, page)| (title_key(&page.title), index))
            .collect();
        Ok(Self { pages, by_title })
    }

    pub(super) fn find(&self, title: &str) -> Option<&Page> {
        let index = self.by_title.get(&title_key(title))?;
        self.pages.get(*index)
    }

    /// Gives the titles that look most like `title`, the closest first.
    pub(super) fn closest_titles(&self, title: &str, limit: usize) -> Vec<&str> {
        let query = title_key(title);
        let mut scored: Vec<(f64, &str)> = self
            .pages
            .iter()
            .map(|page| (title_score(&query, &page.title), page.title.as_str()))
            .collect();
        scored.sort_by(|a, b| b.0.total_cmp(&a.0));
        scored
            .into_iter()
            .take(limit)
            .map(|(_, title)| title)
            .collect()
    }
}

// region:    --- Support

/// Wiki URLs write spaces as underscores, so `Mod_Project_Setup` and `mod project setup` are the same title.
fn title_key(title: &str) -> String {
    title
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// Counts each pair of letters that stand next to each other: "setup" has "se", "et", "tu", "up".
fn bigrams(text: &str) -> HashMap<(char, char), u32> {
    let mut counts = HashMap::new();
    for pair in text.chars().zip(text.chars().skip(1)) {
        *counts.entry(pair).or_insert(0) += 1;
    }
    counts
}

/// Gives 1 when both texts have the same letter pairs and 0 when they share none (the Dice coefficient).
fn similarity(a: &str, b: &str) -> f64 {
    let pairs_a = bigrams(a);
    let pairs_b = bigrams(b);
    let shared: u32 = pairs_a
        .iter()
        .map(|(pair, count)| (*count).min(pairs_b.get(pair).copied().unwrap_or(0)))
        .sum();
    let total: u32 = pairs_a.values().chain(pairs_b.values()).sum();
    if total == 0 {
        return 0.0;
    }
    2.0 * f64::from(shared) / f64::from(total)
}

/// Claude often leaves out the "Arma Reforger:" namespace, so a title is also compared without it.
fn title_score(query: &str, title: &str) -> f64 {
    let key = title_key(title);
    let without_namespace = key.split_once(':').map_or(key.as_str(), |(_, rest)| rest);
    similarity(query, &key).max(similarity(query, without_namespace))
}

// endregion: --- Support
