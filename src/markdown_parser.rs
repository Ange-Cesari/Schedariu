use pulldown_cmark::{html, Options, Parser};

pub struct MarkdownParser {
    options: Options,
}

impl MarkdownParser {
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        Self { options }
    }

    pub fn parse(&self, markdown_input: &str) -> String {
        let parser = Parser::new_ext(markdown_input, self.options);

        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        html_output
    }
}
