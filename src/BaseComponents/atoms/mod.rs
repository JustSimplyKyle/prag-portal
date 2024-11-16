pub mod button;
pub mod center;
pub mod switch;

#[must_use]
pub fn markdown_to_html(s: &str) -> String {
    let parser = pulldown_cmark::Parser::new(s);
    let mut string = String::new();
    pulldown_cmark::html::push_html(&mut string, parser);
    ammonia::clean(&string)
}
