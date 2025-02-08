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

    let decoder = png::Decoder::new(std::fs::File::open("./bouzuya.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    let bytes = &buf[..info.buffer_size()];
    let bytes_per_pixel = info.line_size / info.width as usize;

    let image_stream = lopdf::Stream::new(
        {
            let mut dictionary = lopdf::Dictionary::new();
            dictionary.set("Type", lopdf::Object::Name("XObject".into()));
            dictionary.set("Subtype", lopdf::Object::Name("Image".into()));
            dictionary.set("ColorSpace", lopdf::Object::Name("DeviceRGB".into()));
            dictionary.set("Width", lopdf::Object::Integer(info.width as i64));
            dictionary.set("Height", lopdf::Object::Integer(info.height as i64));
            dictionary.set(
                "BitsPerComponent",
                lopdf::Object::Integer(info.bit_depth as u8 as i64),
            );
            dictionary
        },
        (0..bytes.len() / bytes_per_pixel)
            .flat_map(|i| match info.color_type.samples() {
                1 | 2 => vec![bytes[i * bytes_per_pixel]],
                3 | 4 => vec![
                    bytes[i * bytes_per_pixel],
                    bytes[i * bytes_per_pixel + 1],
                    bytes[i * bytes_per_pixel + 2],
                ],
                _ => unreachable!(),
            })
            .collect::<Vec<u8>>(),
    );

    document
        .insert_image(page_id, image_stream, (0.0, 0.0), (100.0, 100.0))
        .unwrap();

    let file = std::fs::File::create_new("o.pdf").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    document.save_to(&mut writer).unwrap();
}
