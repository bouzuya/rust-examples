use std::{
    fs::File,
    io::{BufWriter, Write},
};

use printpdf::{
    Line, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfLayerReference, PdfPageIndex,
    Point,
};

pub struct MyPdfDocument {
    pdf_document_reference: PdfDocumentReference,
    pdf_page_index: PdfPageIndex,
    pdf_layer_index: PdfLayerIndex,
}

impl MyPdfDocument {
    fn new<P1, P2, S1, S2>(
        document_title: S1,
        initial_page_width: P1,
        initial_page_height: P2,
        initial_layer_name: S2,
    ) -> MyPdfDocument
    where
        P1: Into<Mm>,
        P2: Into<Mm>,
        S1: Into<String>,
        S2: Into<String>,
    {
        let (pdf_document_reference, pdf_page_index, pdf_layer_index) = PdfDocument::new(
            document_title,
            initial_page_width.into(),
            initial_page_height.into(),
            initial_layer_name,
        );
        Self {
            pdf_document_reference,
            pdf_page_index,
            pdf_layer_index,
        }
    }

    pub fn f1(&self) -> anyhow::Result<()> {
        let pdf_layer_reference = self.get_current_layer();
        pdf_layer_reference.add_line(Line {
            points: vec![
                (Point::new(Mm(10.0), Mm(10.0)), false),
                (Point::new(Mm(20.0), Mm(10.0)), false),
            ],
            is_closed: false,
        });
        Ok(())
    }

    pub fn get_current_layer(&self) -> PdfLayerReference {
        let pdf_page_reference = self.pdf_document_reference.get_page(self.pdf_page_index);
        pdf_page_reference.get_layer(self.pdf_layer_index)
    }

    pub fn into_bytes(self) -> anyhow::Result<Vec<u8>> {
        Ok(self.pdf_document_reference.save_to_bytes()?)
    }
}

pub fn main() -> anyhow::Result<()> {
    let doc = MyPdfDocument::new("PDF_Document_title", Mm(247.0), Mm(210.0), "Layer 1");

    doc.f1()?;

    let mut writer = BufWriter::new(File::create("test_working.pdf")?);
    writer.write_all(&doc.into_bytes()?)?;
    Ok(())
}
