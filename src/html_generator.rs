use crate::markdown_parser::MarkdownParser;

pub fn generate_html(markdown_input: &str) -> String {
    let parser = MarkdownParser::new();
    parser.parse(markdown_input)
}