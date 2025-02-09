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
    // #[rustfmt::skip]
    // let buf = vec![
    //     0xFF, 0xFF,
    //     0xFF, 0xFF,
    //     0x00, 0x00,
    //     0x00, 0x00,
    //     0xFF, 0xFF,
    //     0xFF, 0xFF,
    //     0x00, 0x00,
    //     0x00, 0x00,
    //     0xFF, 0xFF,
    //     0xFF, 0xFF,
    //     0x00, 0x00,
    //     0x00, 0x00,
    //     0xFF, 0xFF,
    //     0xFF, 0xFF,
    //     0x00, 0x00,
    //     0x00, 0x00,
    // ];
    // let image_stream = lopdf::Stream::new(
    //      {
    //          let mut dictionary = lopdf::Dictionary::new();
    //          dictionary.set("Type", lopdf::Object::Name("XObject".into()));
    //          dictionary.set("Subtype", lopdf::Object::Name("Image".into()));
    //          // dictionary.set("Filter", lopdf::Object::Name("JPXDeccode".into()));
    //          dictionary.set("ColorSpace", lopdf::Object::Name("DeviceGray".into()));
    //          dictionary.set("Width", lopdf::Object::Integer(16));
    //          dictionary.set("Height", lopdf::Object::Integer(16));
    //          dictionary.set("BitsPerComponent", lopdf::Object::Integer(1));
    //          dictionary
    //      },
    //      buf,
    //  );

    // let image = Image::from_png_file_path("./bouzuya.png").unwrap();
    let image = Image::from_png_file_path("./dummy2.png").unwrap();
    let size = (image.width() as f32, image.height() as f32);
    let stream = image.into_lopdf_stream();
    document
        .insert_image(page_id, stream, (0.0, 0.0), size)
        .unwrap();
    let file = std::fs::File::create_new("o.pdf").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    document.save_to(&mut writer).unwrap();
}
