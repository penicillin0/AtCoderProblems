use anyhow::{anyhow, Result};

use super::AtCoderProblem;

use scraper::{Html, Selector};

pub(super) fn scrape(html: &str, contest_id: &str) -> Result<Vec<AtCoderProblem>> {
    Html::parse_document(html)
        .select(&Selector::parse("tbody").unwrap())
        .next()
        .ok_or_else(|| anyhow!("Failed to parse html."))?
        .select(&Selector::parse("tr").unwrap())
        .map(|tr| {
            let selector = Selector::parse("td").unwrap();
            let mut tds = tr.select(&selector);
            let position = tds
                .next()
                .ok_or_else(|| anyhow!("Failed to parse html."))?
                .text()
                .next()
                .ok_or_else(|| anyhow!("Failed to parse html."))?
                .to_owned();
            let problem = tds.next().ok_or_else(|| anyhow!("Failed to parse html."))?;
            let id = problem
                .select(&Selector::parse("a").unwrap())
                .next()
                .ok_or_else(|| anyhow!("Failed to parse html."))?
                .value()
                .attr("href")
                .ok_or_else(|| anyhow!("Failed to parse html."))?
                .rsplit('/')
                .next()
                .ok_or_else(|| anyhow!("Failed to parse html."))?
                .to_owned();
            let title = problem
                .text()
                .next()
                .ok_or_else(|| anyhow!("Failed to parse html."))?
                .to_owned();
            Ok(AtCoderProblem {
                id,
                contest_id: contest_id.to_owned(),
                title,
                position,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_scrape() {
        let mut file = File::open("test_resources/abc107_tasks").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let problems = scrape(&contents, "abc107").unwrap();
        assert_eq!(
            problems,
            vec![
                AtCoderProblem {
                    id: "abc107_a".to_owned(),
                    contest_id: "abc107".to_owned(),
                    title: "Train".to_owned(),
                    position: "A".to_owned()
                },
                AtCoderProblem {
                    id: "abc107_b".to_owned(),
                    contest_id: "abc107".to_owned(),
                    title: "Grid Compression".to_owned(),
                    position: "B".to_owned()
                },
                AtCoderProblem {
                    id: "arc101_a".to_owned(),
                    contest_id: "abc107".to_owned(),
                    title: "Candles".to_owned(),
                    position: "C".to_owned()
                },
                AtCoderProblem {
                    id: "arc101_b".to_owned(),
                    contest_id: "abc107".to_owned(),
                    title: "Median of Medians".to_owned(),
                    position: "D".to_owned()
                }
            ]
        );
    }

    #[test]
    fn test_scrape_atc002() {
        let mut file = File::open("test_resources/atc002_tasks").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let problems = scrape(&contents, "atc002").unwrap();
        assert_eq!(
            problems,
            vec![
                AtCoderProblem {
                    id: "abc007_3".to_owned(),
                    contest_id: "atc002".to_owned(),
                    title: "幅優先探索".to_owned(),
                    position: "A".to_owned(),
                },
                AtCoderProblem {
                    id: "atc002_b".to_owned(),
                    contest_id: "atc002".to_owned(),
                    title: "n^p mod m".to_owned(),
                    position: "B".to_owned(),
                },
                AtCoderProblem {
                    id: "atc002_c".to_owned(),
                    contest_id: "atc002".to_owned(),
                    title: "最適二分探索木".to_owned(),
                    position: "C".to_owned(),
                },
            ]
        )
    }
}
