use reqwest::Client;
use rss::Channel;
use scraper::{Html, Selector};
use serde::Serialize;
use thiserror::Error;

// RSS feed
// http://feeds.feedblitz.com/italian-word-of-the-day
const ITALIAN_FEED: &str = "http://feeds.feedblitz.com/italian-word-of-the-day";

#[derive(Debug, Default, Serialize)]
pub(crate) struct ItalianWord {
    word: String,
    part_of_speech: String,
    italian_sentence: String,
    english_sentence: String,
}

#[derive(Debug, Error)]
pub(crate) enum ItalianError {
    #[error("Error fetching Italian word of the day feed")]
    NetworkError(#[from] reqwest::Error),
    #[error("Error parsing rss feed")]
    RssError(#[from] rss::Error),
    #[error("Error no word found in rss feed")]
    WordError,
    #[error("Error no description found in rss feed")]
    DescriptionError,
}

pub(crate) async fn italian_word(client: &Client) -> Result<ItalianWord, ItalianError> {
    let mut it: ItalianWord = ItalianWord::default();

    let resp = client.get(ITALIAN_FEED).send().await?.bytes().await?;
    let rss = Channel::read_from(&resp[..])?;

    let word_item = rss.items.first().ok_or(ItalianError::WordError)?;
    let italian_word = word_item.title().unwrap_or("WORD NOT FOUND");
    it.word = italian_word.to_string();

    let unparsed_italian_desc = word_item.description();

    match unparsed_italian_desc {
        None => Err(ItalianError::DescriptionError),
        Some(desc) => {
            let fragment = Html::parse_fragment(desc);
            let row_selector = Selector::parse("tr").unwrap();
            let td_selector = Selector::parse("td").unwrap();
            let th_selector = Selector::parse("th").unwrap();

            for element in fragment.select(&row_selector) {
                let table_parse: Vec<(scraper::ElementRef<'_>, scraper::ElementRef<'_>)> = element
                    .select(&th_selector)
                    .zip(
                        element
                            .select(&td_selector)
                            .collect::<Vec<scraper::ElementRef<'_>>>(),
                    )
                    .collect();

                for (header, value) in table_parse {
                    match header.inner_html().as_str().to_lowercase().as_str() {
                        "part of speech:" => it.part_of_speech = value.inner_html(),
                        "example sentence:" => it.italian_sentence = value.inner_html(),
                        "sentence meaning:" => it.english_sentence = value.inner_html(),
                        _ => println!("WARNING: UNKNOWN TABLE PART!"),
                    }
                }
            }

            Ok(it)
        }
    }
}
