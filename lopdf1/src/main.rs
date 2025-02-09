mod bits_per_component;
mod color_space;
mod image;
mod unit;

use image::Image;
use unit::F32Ext as _;

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
    let width = (image.width() as f32).px();
    let height = (image.height() as f32).px();
    let stream = image.into_lopdf_stream();
    let x = 210.0.mm().to_px() - width;
    let y = 297.0.mm().to_px() - height;
    document
        .insert_image(
            page_id,
            stream,
            (x.to_f32(), y.to_f32()),
            (width.to_f32(), height.to_f32()),
        )
        .unwrap();
    let file = std::fs::File::create_new("o.pdf").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    document.save_to(&mut writer).unwrap();
}
