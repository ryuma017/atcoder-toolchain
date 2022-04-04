use anyhow::Result;
use reqwest::Url;
use scraper::{Html, Selector};

const ATCODER_BASE_URL: &str = "https://atcoder.jp";

pub struct AtCoderScraper {
    client: reqwest::Client,
}

impl Default for AtCoderScraper {
    fn default() -> Self {
        Self::new()
    }
}

impl AtCoderScraper {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn task_ids(&self, contest_name: &str) -> Result<Vec<String>> {
        let target_url = Url::parse_with_params(
            Url::parse(ATCODER_BASE_URL)?
                .join(&format!("contests/{}", contest_name))?
                .as_str(),
            &[("lang", "en")],
        )?;
        let html = self.client.get(target_url).send().await?.text().await?;

        let document = Html::parse_document(&html);
        let tbody_selector =
            Selector::parse("#contest-statement > .lang > .lang-en tbody").unwrap();
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        // inner HTML of `tbody`
        // <tr>
        //   <td style="text-align:center">A</td>
        //   <td style="text-align:center">?</td>
        // </tr>
        // <tr>
        //   <td style="text-align:center">B</td>
        //   <td style="text-align:center">?</td>
        // </tr>
        // <tr>
        //   <td style="text-align:center">C</td>
        //   <td style="text-align:center">?</td>
        // </tr>
        // <tr>
        //   <td style="text-align:center">D</td>
        //   <td style="text-align:center">?</td>
        // </tr>

        Ok(document
            .select(&tbody_selector)
            // .inspect(|tbody| println!("tbody: {}", tbody.inner_html()))
            .next()
            .expect("table does not exist")
            .select(&tr_selector)
            // .inspect(|tr| println!("tr: {}", tr.inner_html()))
            .map(|tr| {
                tr.select(&td_selector)
                    // .inspect(|td| println!("td: {}", td.inner_html()))
                    .find_map(|td| Some(td.inner_html()))
                    .expect("could not find the task ids")
                    .to_lowercase()
            })
            .collect())
    }
}
