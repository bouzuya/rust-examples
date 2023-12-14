mod my_pdf;

use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

#[allow(dead_code)]
fn f1() -> anyhow::Result<()> {
    let (doc, _page1, _layer1) =
        PdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");
    let (_page2, _layer1) = doc.add_page(Mm(10.0), Mm(250.0), "Page 2, Layer 1");

    doc.save(&mut BufWriter::new(
        File::create("test_working.pdf").unwrap(),
    ))?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    my_pdf::main()?;
    Ok(())
}
