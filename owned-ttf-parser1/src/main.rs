mod wrap;

use std::{fs::File, io::Read};

use anyhow::Context;
use owned_ttf_parser::{AsFaceRef, OwnedFace};

fn main() -> anyhow::Result<()> {
    let mut file = File::open("MPLUS1p-Regular.ttf")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let face = OwnedFace::from_vec(buf, 0)?;
    let face = face.as_face_ref();

    let pt = 16;

    let result = "abcあいう"
        .chars()
        .map(|c| -> anyhow::Result<_> {
            let glyph_id = face
                .glyph_index(c)
                .with_context(|| format!("char '{}' not found", c))?;
            let hor_advance = face
                .glyph_hor_advance(glyph_id)
                .with_context(|| format!("char '{}' has no horizontal advance", c))?;

            Ok((
                c,
                hor_advance,
                (hor_advance as usize * pt * 72_usize) as f64
                    / (72_usize * face.units_per_em() as usize) as f64,
            ))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    // <https://learn.microsoft.com/ja-jp/typography/opentype/spec/ttch01#converting-funits-to-pixels>
    assert_eq!((550 * 18 * 72) as f64 / (72 * 2048) as f64, 4.833984375);

    assert_eq!(face.units_per_em(), 1000);
    assert_eq!(
        result,
        vec![
            ('a', 548, 8.768),
            ('b', 578, 9.248),
            ('c', 522, 8.352),
            ('あ', 1000, 16.0),
            ('い', 1000, 16.0),
            ('う', 1000, 16.0),
        ]
    );

    println!("ascender                     = {}", face.ascender());
    println!("capital_height      (option) = {:?}", face.capital_height());
    println!("descender                    = {}", face.descender());
    println!(
        "global_bounding_box (option) = {:?}",
        face.global_bounding_box()
    );
    println!("height                       = {}", face.height());
    println!("line_gap                     = {}", face.line_gap());

    #[allow(unused_variables)]
    let print_glyph_metrics = |c: char| {
        let glyph_id = face
            .glyph_index(c)
            .unwrap_or_else(|| panic!("{:?} has no glyph_id", c));
        println!("c = {:?}", c);
        println!(
            "glyph_hor_advance      = {:?}",
            face.glyph_hor_advance(glyph_id)
        );
        println!(
            "glyph_hor_side_bearing = {:?}",
            face.glyph_hor_side_bearing(glyph_id)
        );
        println!(
            "glyph_ver_advance      = {:?}",
            face.glyph_ver_advance(glyph_id)
        );
        println!(
            "glyph_ver_side_bearing = {:?}",
            face.glyph_ver_side_bearing(glyph_id)
        );
        println!(
            "glyph_y_origin         = {:?}",
            face.glyph_y_origin(glyph_id)
        );
    };

    print_glyph_metrics('a');
    print_glyph_metrics('b');
    print_glyph_metrics('c');
    print_glyph_metrics('あ');

    assert!(face.glyph_index('a').is_some());
    assert!(face.glyph_index('あ').is_some());
    assert!(face.glyph_index('\n').is_none());
    assert!(face.glyph_index('\r').is_none());
    assert!(face.glyph_index(' ').is_some());

    // a (0.548) + b (0.578) + c (0.522) = 1.648
    let f = |s: &str, w: f32| -> String {
        wrap::wrap(s, w, |c: char| -> f32 {
            // '\n' などは glyph_id が取得できない
            let hor_advance = face
                .glyph_index(c)
                .and_then(|glyph_id| face.glyph_hor_advance(glyph_id))
                .map(usize::from)
                .unwrap_or_default();
            hor_advance as f32 / face.units_per_em() as f32
        })
    };
    assert_eq!(f("abc", 1.648), "abc");
    assert_eq!(f("abc", 1.647), "ab\nc");

    Ok(())
}
