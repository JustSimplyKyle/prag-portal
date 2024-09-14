pub mod button;
pub mod switch;

pub fn markdown_to_html(s: &str) -> String {
    let parser = pulldown_cmark::Parser::new(s);
    let mut string = String::new();
    pulldown_cmark::html::push_html(&mut string, parser);
    string
}
