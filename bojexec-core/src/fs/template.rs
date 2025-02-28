use crate::problem::Problem;
use handlebars::Handlebars;

const DESCRIPTION_TEMPLATE: &str = r#"
# {{title}}

{{description.problem}}

## Input

{{description.input}}

## Output

{{description.output}}
"#;

pub fn render_markdown(problem: &Problem) -> Result<String, Box<dyn std::error::Error>> {
    let handlebars = Handlebars::new();
    let markdown = handlebars.render_template(DESCRIPTION_TEMPLATE, problem)?;

    Ok(markdown)
}
