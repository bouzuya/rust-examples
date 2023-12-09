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
    Ok(())
}
