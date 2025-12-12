fn ex1() {
    let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();

    let mut names = syntax_set
        .syntaxes()
        .iter()
        .map(|it| it.name.clone())
        .collect::<Vec<String>>();
    names.sort();
    assert_ne!(names, Vec::<String>::new());

    assert!(syntax_set.find_syntax_by_token("Rust").is_some());
    assert!(syntax_set.find_syntax_by_token("rust").is_some());
    assert!(syntax_set.find_syntax_by_token("rs").is_some());

    assert!(syntax_set.find_syntax_by_token("TypeScript").is_none());
    assert!(syntax_set.find_syntax_by_token("typescript").is_none());
    assert!(syntax_set.find_syntax_by_token("ts").is_none());

    assert!(syntax_set.find_syntax_by_token("Haskell").is_some());
    assert!(syntax_set.find_syntax_by_token("haskell").is_some());
    assert!(syntax_set.find_syntax_by_token("hs").is_some());

    assert!(syntax_set.find_syntax_by_token("JavaScript").is_some());
    assert!(syntax_set.find_syntax_by_token("javascript").is_some());
    assert!(syntax_set.find_syntax_by_token("js").is_some());
    assert!(syntax_set.find_syntax_by_token("cjs").is_none());
    assert!(syntax_set.find_syntax_by_token("mjs").is_none());

    let theme_set = syntect::highlighting::ThemeSet::load_defaults();
    assert_ne!(
        theme_set.themes.keys().collect::<Vec<&String>>(),
        Vec::<&String>::new()
    );

    assert!(theme_set.themes.contains_key("InspiredGitHub"));
    assert!(theme_set.themes.contains_key("base16-ocean.dark"));

    let html = syntect::html::highlighted_html_for_string(
        "fn main() {\n    println!(\"Hello, world!\");\n}",
        &syntax_set,
        syntax_set.find_syntax_by_token("rust").unwrap(),
        &theme_set.themes["base16-ocean.dark"],
    )
    .unwrap();
    assert_eq!(
        html,
        "<pre style=\"background-color:#2b303b;\">\n<span style=\"color:#b48ead;\">fn </span><span style=\"color:#8fa1b3;\">main</span><span style=\"color:#c0c5ce;\">() {\n</span><span style=\"color:#c0c5ce;\">    println!(&quot;</span><span style=\"color:#a3be8c;\">Hello, world!</span><span style=\"color:#c0c5ce;\">&quot;);\n</span><span style=\"color:#c0c5ce;\">}</span></pre>\n"
    );

    let mut classed_html_generator = syntect::html::ClassedHTMLGenerator::new_with_class_style(
        syntax_set.find_syntax_by_token("rust").unwrap(),
        &syntax_set,
        syntect::html::ClassStyle::Spaced,
    );
    for line in
        syntect::util::LinesWithEndings::from("fn main() {\n    println!(\"Hello, world!\");\n}")
    {
        classed_html_generator
            .parse_html_for_line_which_includes_newline(line)
            .unwrap();
    }
    let html = classed_html_generator.finalize();
    assert_eq!(
        html,
        "<span class=\"source rust\"><span class=\"meta function rust\"><span class=\"meta function rust\"><span class=\"storage type function rust\">fn</span> </span><span class=\"entity name function rust\">main</span></span><span class=\"meta function rust\"><span class=\"meta function parameters rust\"><span class=\"punctuation section parameters begin rust\">(</span></span><span class=\"meta function rust\"><span class=\"meta function parameters rust\"><span class=\"punctuation section parameters end rust\">)</span></span></span></span><span class=\"meta function rust\"> </span><span class=\"meta function rust\"><span class=\"meta block rust\"><span class=\"punctuation section block begin rust\">{</span>\n    <span class=\"support macro rust\">println!</span><span class=\"meta group rust\"><span class=\"punctuation section group begin rust\">(</span></span><span class=\"meta group rust\"><span class=\"string quoted double rust\"><span class=\"punctuation definition string begin rust\">&quot;</span>Hello, world!<span class=\"punctuation definition string end rust\">&quot;</span></span></span><span class=\"meta group rust\"><span class=\"punctuation section group end rust\">)</span></span><span class=\"punctuation terminator rust\">;</span>\n</span><span class=\"meta block rust\"><span class=\"punctuation section block end rust\">}</span></span></span></span>"
    );

    let css = syntect::html::css_for_theme_with_class_style(
        &theme_set.themes["base16-ocean.dark"],
        syntect::html::ClassStyle::Spaced,
    )
    .unwrap();
    assert_ne!(css, "");
}

fn ex2() {
    let syntax_dir = std::path::PathBuf::from("syntaxes");
    let syntax_dir = syntax_dir.canonicalize().unwrap();
    // println!("syntax_dir: {:?}", syntax_dir.display());
    let mut syntax_set_builder = syntect::parsing::SyntaxSetBuilder::new();
    syntax_set_builder.add_plain_text_syntax();
    syntax_set_builder
        .add_from_folder(&syntax_dir, true)
        .unwrap();
    let syntax_set = syntax_set_builder.build();
    assert_eq!(syntax_set.syntaxes().len(), 52);

    assert!(syntax_set.find_syntax_by_token("TypeScript").is_some());
    assert!(syntax_set.find_syntax_by_token("typescript").is_some());
    assert!(syntax_set.find_syntax_by_token("ts").is_some());
}

fn main() {
    ex1();
    ex2();
}
