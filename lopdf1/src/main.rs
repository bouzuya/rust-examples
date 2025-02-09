mod bits_per_component;
mod color_space;
mod image;

use image::Image;

fn main() {
    let mut document = lopdf::Document::load("./a-output.pdf").unwrap();
    println!("document.version = {:?}", document.version);
    println!("document = {:?}", document);
    for page_object_id in document.page_iter() {
        // println!("object_id = {:?}", page_object_id);
        let page_contents = document.get_page_contents(page_object_id);
        for page_content_object_id in page_contents {
            println!("{:?}", page_content_object_id);
            let object = document.get_object(page_content_object_id).unwrap();
            println!("{:?}", object);
        }
    }
    document
        .replace_text(1, "Hello, World!", "bouzuya")
        .unwrap();
    let page_id = *document.get_pages().get(&1_u32).unwrap();
    // let image = Image::from_png_file_path("./bouzuya.png").unwrap();
    let image = Image::from_png_file_path("./dummy2.png").unwrap();
    let size = (image.width() as f32, image.height() as f32);
    let stream = image.into_lopdf_stream();
    let x = mm_to_px(210.0 - px_to_mm(size.0));
    let y = mm_to_px(297.0 - px_to_mm(size.1));
    document
        .insert_image(page_id, stream, (x, y), size)
        .unwrap();
    let file = std::fs::File::create_new("o.pdf").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    document.save_to(&mut writer).unwrap();
}

fn mm_to_px(mm: f32) -> f32 {
    let mmpi = 25.4; // mm per inch
    let dpi = 72.0; // dot (px) per inch
    let px = mm / mmpi * dpi;
    px
}

fn px_to_mm(px: f32) -> f32 {
    let mmpi = 25.4; // mm per inch
    let dpi = 72.0; // dot (px) per inch
    let mm = px / dpi * mmpi;
    mm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mm_to_px() {
        // A4 = 210mm x 297mm
        assert_eq!(mm_to_px(210.0), 595.2756);
        assert_eq!(mm_to_px(297.0), 841.88983);
    }

    #[test]
    fn test_px_to_mm() {
        // A4 = 210mm x 297mm
        assert_eq!(px_to_mm(595.2756), 210.0);
        assert_eq!(px_to_mm(841.88983), 297.0);
    }
}
