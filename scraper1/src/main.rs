fn main() {
    println!("Hello, world!");
}

#[test]
fn parsing_a_document() {
    use scraper::Html;

    let html = r#"
    <!DOCTYPE html>
    <meta charset="utf-8">
    <title>Hello, world!</title>
    <h1 class="foo">Hello, <i>world!</i></h1>
"#;

    let document = Html::parse_document(html);
    assert_ne!(format!("{:?}", document), "");
}

#[test]
fn parsing_a_fragment() {
    use scraper::Html;
    let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
    assert_ne!(format!("{:?}", fragment), "");
}

#[test]
fn parsing_a_selector() {
    use scraper::Selector;
    let selector = Selector::parse("h1.foo").unwrap();
    assert_ne!(format!("{:?}", selector), "");
}

#[test]
fn selecting_elements() {
    use scraper::{Html, Selector};

    let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

    let fragment = Html::parse_fragment(html);
    let selector = Selector::parse("li").unwrap();

    for element in fragment.select(&selector) {
        assert_eq!("li", element.value().name());
    }
}

#[test]
fn selecting_descendent_elements() {
    use scraper::{Html, Selector};

    let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

    let fragment = Html::parse_fragment(html);
    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let ul = fragment.select(&ul_selector).next().unwrap();
    for element in ul.select(&li_selector) {
        assert_eq!("li", element.value().name());
    }
}

#[test]
fn accessing_element_attributes() {
    use scraper::{Html, Selector};

    let fragment = Html::parse_fragment(r#"<input name="foo" value="bar">"#);
    let selector = Selector::parse(r#"input[name="foo"]"#).unwrap();

    let input = fragment.select(&selector).next().unwrap();
    assert_eq!(Some("bar"), input.value().attr("value"));
}

#[test]
fn serializing_html_and_inner_html() {
    use scraper::{Html, Selector};

    let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
    let selector = Selector::parse("h1").unwrap();

    let h1 = fragment.select(&selector).next().unwrap();

    assert_eq!("<h1>Hello, <i>world!</i></h1>", h1.html());
    assert_eq!("Hello, <i>world!</i>", h1.inner_html());
}

#[test]
fn accessing_descendent_text() {
    use scraper::{Html, Selector};

    let fragment = Html::parse_fragment("<h1>Hello, <i>world!</i></h1>");
    let selector = Selector::parse("h1").unwrap();

    let h1 = fragment.select(&selector).next().unwrap();
    let text = h1.text().collect::<Vec<_>>();

    assert_eq!(vec!["Hello, ", "world!"], text);
}
