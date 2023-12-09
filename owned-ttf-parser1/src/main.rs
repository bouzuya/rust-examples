use std::{fs::File, io::Read};

use anyhow::Context;
use owned_ttf_parser::{AsFaceRef, OwnedFace};

fn main() -> anyhow::Result<()> {
    let mut file = File::open("MPLUS1p-Regular.ttf")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let face = OwnedFace::from_vec(buf, 0)?;
    let face = face.as_face_ref();

    let result = "abcあいう"
        .chars()
        .map(|c| -> anyhow::Result<_> {
            let glyph_id = face
                .glyph_index(c)
                .with_context(|| format!("char '{}' not found", c))?;
            let hor_advance = face
                .glyph_hor_advance(glyph_id)
                .with_context(|| format!("char '{}' has no horizontal advance", c))?;
            Ok((c, hor_advance))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    assert_eq!(
        result,
        vec![
            ('a', 548_u16),
            ('b', 578_u16),
            ('c', 522_u16),
            ('あ', 1000_u16),
            ('い', 1000_u16),
            ('う', 1000_u16),
        ]
    );
    Ok(())
}
