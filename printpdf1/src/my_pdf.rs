use std::{
    fs::File,
    io::{BufWriter, Write},
};

use printpdf::{
    Line, Mm, PdfDocument, PdfDocumentReference, PdfLayerIndex, PdfLayerReference, PdfPageIndex,
    Point,
};

#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub height: Mm,
    pub width: Mm,
}

pub struct MyPdfDocument {
    pdf_document_reference: PdfDocumentReference,
    pdf_page_index: PdfPageIndex,
    pdf_layer_index: PdfLayerIndex,
}

impl MyPdfDocument {
    pub fn new<P1, P2, S1, S2>(
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

    pub fn add_horizontal_line<P>(&self, point: Point, width: P)
    where
        P: Into<Mm>,
    {
        self.add_line(&[
            point,
            Point::new(Mm::from(point.x) + width.into(), Mm::from(point.y)),
        ]);
    }

    pub fn add_rectangle(&self, point: Point, size: Size) {
        let (x, y, w, h) = (
            Mm::from(point.x),
            Mm::from(point.y),
            size.width,
            size.height,
        );
        self.add_line(&[
            point,
            Point::new(x, y + h),
            Point::new(x + w, y + h),
            Point::new(x + w, y),
            point,
        ]);
    }

    pub fn add_vertical_line<P>(&self, point: Point, height: P)
    where
        P: Into<Mm>,
    {
        self.add_line(&[
            point,
            Point::new(Mm::from(point.x), Mm::from(point.y) + height.into()),
        ]);
    }

    pub fn into_bytes(self) -> anyhow::Result<Vec<u8>> {
        Ok(self.pdf_document_reference.save_to_bytes()?)
    }

    // private methods

    fn add_line(&self, points: &[Point]) {
        let layer = self.get_current_layer();
        layer.add_line(Line {
            points: points
                .iter()
                .copied()
                .map(|point| (point, false))
                .collect::<Vec<(Point, bool)>>(),
            is_closed: false,
        });
    }

    fn get_current_layer(&self) -> PdfLayerReference {
        let pdf_page_reference = self.pdf_document_reference.get_page(self.pdf_page_index);
        pdf_page_reference.get_layer(self.pdf_layer_index)
    }
}

pub fn main() -> anyhow::Result<()> {
    let doc = MyPdfDocument::new("PDF_Document_title", Mm(210.0), Mm(297.0), "Layer 1");

    doc.add_horizontal_line(Point::new(Mm(10.0), Mm(10.0)), Mm(10.0));
    doc.add_vertical_line(Point::new(Mm(10.0), Mm(10.0)), Mm(10.0));
    doc.add_rectangle(
        Point::new(Mm(15.0), Mm(15.0)),
        Size {
            height: Mm(10.0),
            width: Mm(10.0),
        },
    );

    let mut writer = BufWriter::new(File::create("test_working.pdf")?);
    writer.write_all(&doc.into_bytes()?)?;
    Ok(())
}
