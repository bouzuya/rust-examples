fn main() {
    ex1();
    ex2();
    ex3();
}

fn ex1() {
    let tree = resvg::usvg::Tree::from_str(
        include_str!("../bouzuya.svg"),
        &resvg::usvg::Options::default(),
    )
    .unwrap();
    let transform = resvg::usvg::Transform::default();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(4096, 4096).unwrap();
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    pixmap.save_png("bouzuya.png").unwrap();
}

fn ex2() {
    let tree = resvg::usvg::Tree::from_str(
        include_str!("../bouzuya.svg"),
        &resvg::usvg::Options::default(),
    )
    .unwrap();
    let original_size = tree.size();
    assert_eq!(original_size.width(), 4096.0);
    assert_eq!(original_size.height(), 4096.0);

    for size in [100_u16, 200_u16, 600_u16, 1200_u16] {
        let new_size = resvg::usvg::Size::from_wh(f32::from(size), f32::from(size)).unwrap();
        let transform = resvg::usvg::Transform::from_scale(
            new_size.width() / original_size.width(),
            new_size.height() / original_size.height(),
        );
        let mut pixmap = resvg::tiny_skia::Pixmap::new(u32::from(size), u32::from(size)).unwrap();
        resvg::render(&tree, transform, &mut pixmap.as_mut());
        pixmap
            .save_png(&format!("v4-bouzuya-{}x{}.png", size, size))
            .unwrap();
    }
}

fn ex3() {
    let tree = resvg::usvg::Tree::from_str(
        include_str!("../bouzuya.svg"),
        &resvg::usvg::Options::default(),
    )
    .unwrap();
    let original_size = tree.size();
    assert_eq!(original_size.width(), 4096.0);
    assert_eq!(original_size.height(), 4096.0);

    for size in [100_u16, 200_u16, 600_u16, 1200_u16] {
        let new_size = resvg::usvg::Size::from_wh(f32::from(size), f32::from(size)).unwrap();
        let transform = resvg::usvg::Transform::from_scale(
            new_size.width() / original_size.width(),
            new_size.height() / original_size.height(),
        );
        let mut pixmap = resvg::tiny_skia::Pixmap::new(u32::from(size), u32::from(size)).unwrap();
        resvg::render(&tree, transform, &mut pixmap.as_mut());

        let mut encoder = png::Encoder::new(
            std::fs::File::create(&format!("v5-bouzuya-{}x{}.png", size, size)).unwrap(),
            u32::from(size),
            u32::from(size),
        );
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(pixmap.data()).unwrap();
    }
}
