use crate::crawl::agent::UserAgent;
use crate::problem::{Description, Problem, TestCase};
use reqwest;
use scraper::{Html, Selector};
use std::error::Error;

mod agent;

fn extract_to_markdown(document: &Html, selector: &Selector) -> String {
    let problem_html = document
        .select(selector)
        .next()
        .map(|el| el.inner_html())
        .unwrap_or_default();

    html2md::parse_html(&problem_html)
}

fn parse_baekjoon_body(body: String) -> (String, Description, Vec<TestCase>) {
    let document = Html::parse_document(&body);

    let title_selector = Selector::parse("#problem_title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    // TODO: Implement metadata parser

    let problem_markdown =
        extract_to_markdown(&document, &Selector::parse("#problem_description").unwrap());
    let input_markdown =
        extract_to_markdown(&document, &Selector::parse("#problem_input").unwrap());
    let output_markdown =
        extract_to_markdown(&document, &Selector::parse("#problem_output").unwrap());

    let testcase_selector = Selector::parse(r#"[id^="sample-input-"]"#).unwrap();
    let mut testcases = Vec::new();
    for (index, element) in document.select(&testcase_selector).enumerate() {
        let output_selector = Selector::parse(&format!("#sample-output-{}", index + 1)).unwrap();
        testcases.push(TestCase {
            input: element
                .text()
                .collect::<Vec<_>>()
                .join("")
                .trim()
                .to_string(),
            output: document
                .select(&output_selector)
                .next()
                .map(|el| el.text().collect::<Vec<_>>().join("").trim().to_string())
                .unwrap_or_default(),
        });
    }

    (
        title,
        Description {
            problem: problem_markdown,
            input: input_markdown,
            output: output_markdown,
        },
        testcases,
    )
}

pub async fn get_baekjoon_problem(id: u32) -> Result<Problem, Box<dyn Error>> {
    let url = format!("https://www.acmicpc.net/problem/{}", id);

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("User-Agent", os_info::get().os_type().get_user_agent())
        .send()
        .await?;

    let (title, description, testcases) = parse_baekjoon_body(res.text().await?);
    Ok(Problem {
        id,
        title,
        description,
        testcases,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_baekjoon_problem_network() {
        let problem_id = 1000;
        let result = get_baekjoon_problem(problem_id).await;
        assert!(result.is_ok(), "get_problem returned an error");

        let problem = result.unwrap();
        println!("{:#?}", problem);
        assert!(
            !problem.title.is_empty(),
            "Problem title should not be empty"
        );
        assert!(
            !problem.description.problem.is_empty(),
            "Problem description should not be empty"
        );
        assert!(
            !problem.testcases.is_empty(),
            "At least one sample testcase should be present"
        );
    }
}
