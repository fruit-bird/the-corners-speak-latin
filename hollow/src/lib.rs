use anyhow::{anyhow, Result};
use rand::{seq::SliceRandom, Rng};

const WIKI_URL_PREFIX: &str = "https://en.wikipedia.org/wiki/";
const WIKI_API_URL: &str = "https://en.wikipedia.org/w/api.php";

pub struct Hollow<'a> {
    first: &'a str,
    second: &'a str,
    second_language: &'a str,
}

impl Default for Hollow<'_> {
    fn default() -> Self {
        Self {
            first: "Rumpelstiltskin",
            second: "Moon landing conspiracies",
            second_language: "ja",
        }
    }
}

impl<'a> Hollow<'a> {
    pub fn new(first: &'a str, second: &'a str, second_language: &'a str) -> Self {
        Self {
            first,
            second,
            second_language,
        }
    }

    pub async fn run(&self) -> Result<String> {
        let (entries_1, entries_2) =
            tokio::try_join!(get_entries(self.first), get_entries(self.second))?;

        let (translation_1, translation_2) = tokio::try_join!(
            translator(&entries_1, self.second_language),
            translator(&entries_2, self.second_language)
        )?;

        let mut entries = [entries_1, entries_2, translation_1, translation_2].concat();
        entries.shuffle(&mut rand::thread_rng());

        Ok(entries.join(" "))
    }
}

#[inline]
async fn get_wiki_article(query: &str) -> Result<String> {
    // https://en.wikipedia.org/w/api.php?action=query&format=json&prop=extracts&exintro=&explaintext=&titles=Artificial%20intelligence
    let params = serde_json::json! {
        {
            "action": "query",
            "format": "json",
            "redirects": "resolve",
            "exlimit": 0,
            "explaintext": 1,
            "titles": query.strip_prefix(WIKI_URL_PREFIX).unwrap_or(query),
            "prop": "extracts",
        }
    };

    let client = reqwest::Client::new();
    let response = client.get(WIKI_API_URL).query(&params).send().await?;
    let response_text = response.text().await?;
    let parsed_response = serde_json::from_str::<serde_json::Value>(&response_text)?;

    let pages = parsed_response["query"]["pages"]
        .as_object()
        .ok_or(anyhow!("No pages returned"))?;
    let page = pages.iter().next().ok_or(anyhow!("No pages returned"))?.1;
    let article_text = page["extract"]
        .as_str()
        .ok_or(anyhow!("Could not extract content"))?
        .to_string();

    Ok(article_text)
}

async fn get_entries(query: &str) -> Result<Vec<String>> {
    let article_text = get_wiki_article(query).await?;

    let vec_text = article_text
        .lines()
        .filter_map(|s| {
            match s.len() < 5 || s.contains('\n') || s.starts_with('=') || s.starts_with(' ') {
                true => None,
                false => Some(
                    s.split(' ')
                        .skip(2) // most first words are the/on...
                        .step_by(rand::thread_rng().gen_range(2..6))
                        .take(6) // 4 or 5 seems to be the spot
                        .collect::<Vec<&str>>()
                        .join(" "),
                ),
            }
        })
        .step_by(rand::thread_rng().gen_range(2..5))
        .take(rand::thread_rng().gen_range(30..40))
        .collect();

    Ok(vec_text)
}

async fn translator(entries: &[String], second_language: &str) -> Result<Vec<String>> {
    let translation = google_translator::translate(entries, "auto", second_language)
        .await?
        .output_text
        .iter()
        .filter_map(|v| v.first()) // take first translation, remove alternatives
        .filter(|s| !s.contains("\n"))
        .map(ToString::to_string)
        .collect();

    Ok(translation)
}
