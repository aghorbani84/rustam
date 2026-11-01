use pulldown_cmark::{Options, Parser, html};

pub fn parse_markdown(input: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
