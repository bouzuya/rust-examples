use std::fs;

// <https://commons.wikimedia.org/wiki/SVG_examples>
fn svg1() -> &'static str {
    r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
        <svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="120" height="120">
            <rect x="14" y="23" width="200" height="50" fill="lime" stroke="black" />
        </svg>"#
}

fn svg2() -> &'static str {
    r#"<?xml version="1.0" encoding="UTF-8"?>
        <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
        <svg xmlns="http://www.w3.org/2000/svg" version="1.1" width="120" height="120">
            <text>foo</text>
        </svg>"#
}

fn main() -> anyhow::Result<()> {
    let pdf1 = svg2pdf::convert_str(svg1(), Default::default())?;
    fs::write("target/svg1.pdf", pdf1)?;

    // text is not supported
    let pdf2 = svg2pdf::convert_str(svg2(), Default::default())?;
    fs::write("target/svg2.pdf", pdf2)?;
    Ok(())
}
