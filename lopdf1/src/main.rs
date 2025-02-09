mod bits_per_component;
mod color_space;
mod image;
mod unit;

use image::Image;
use unit::{F32Ext as _, Px};

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

    // insert image
    {
        let page_id = *document.get_pages().get(&1_u32).unwrap();
        // let image = Image::from_png_file_path("./bouzuya.png").unwrap();
        let image = Image::from_png_file_path("./dummy2.png").unwrap();
        let width = (image.width() as f32).px();
        let height = (image.height() as f32).px();
        let stream = image.into_lopdf_stream();
        let x = 210.0.mm().to_px() - width;
        let y = 297.0.mm().to_px() - height;
        insert_image(&mut document, page_id, stream, (x, y), (width, height)).unwrap();
    }

    let file = std::fs::File::create_new("o.pdf").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    document.save_to(&mut writer).unwrap();
}

fn insert_image(
    document: &mut ::lopdf::Document,
    page_id: ::lopdf::ObjectId,
    img_object: ::lopdf::Stream,
    (x, y): (Px, Px),
    (width, height): (Px, Px),
) -> Result<(), ::lopdf::Error> {
    let position = (x.to_f32(), y.to_f32());
    let size = (width.to_f32(), height.to_f32());
    document.insert_image(page_id, img_object, position, size)
}
