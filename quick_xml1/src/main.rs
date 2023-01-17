fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
    use quick_xml::writer::Writer;
    use quick_xml::Reader;

    #[test]
    fn test_reader() -> anyhow::Result<()> {
        let sitemap_xml = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        let mut reader = Reader::from_str(sitemap_xml);
        match reader.read_event()? {
            Event::Decl(_) => {}
            _ => unreachable!(),
        }

        // read url
        #[derive(Debug, Eq, PartialEq)]
        enum Context {
            Root,
            Urlset,
            Url,
            Loc,
        }
        #[derive(Debug)]
        struct Url<'a> {
            loc: Option<BytesText<'a>>,
            // ...
        }
        let mut url = Url { loc: None };
        let mut context = Context::Root;
        loop {
            match reader.read_event()? {
                Event::Start(s) => match s.name().as_ref() {
                    b"urlset" => {
                        if context != Context::Root {
                            todo!()
                        }
                        context = Context::Urlset;
                    }
                    b"url" => {
                        if context != Context::Urlset {
                            todo!()
                        }
                        context = Context::Url;
                    }
                    b"loc" => {
                        if context != Context::Url {
                            todo!()
                        }
                        context = Context::Loc;
                    }
                    _ => unreachable!(),
                },
                Event::End(e) => match e.name().as_ref() {
                    b"urlset" => {
                        if context != Context::Urlset {
                            todo!()
                        }
                        context = Context::Root;
                    }
                    b"url" => {
                        if context != Context::Url {
                            todo!()
                        }
                        context = Context::Urlset;
                        assert_eq!(
                            format!("{:?}", url),
                            r#"Url { loc: Some(BytesText { content: Borrowed("http://www.example.com/") }) }"#
                        );
                    }
                    b"loc" => {
                        if context != Context::Loc {
                            todo!()
                        }
                        context = Context::Url;
                    }
                    _ => unreachable!(),
                },
                Event::Eof => break,
                Event::Text(t) => {
                    if context != Context::Loc {
                        todo!()
                    }
                    url.loc = Some(t);
                }
                _ => unreachable!(),
            };
        }
        Ok(())
    }

    #[test]
    fn test_writer() -> anyhow::Result<()> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        writer.write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)))?;
        let mut elm = BytesStart::new("urlset");
        elm.push_attribute(("xmlns", "http://www.sitemaps.org/schemas/sitemap/0.9"));
        writer.write_event(Event::Start(elm))?;

        writer.write_event(Event::Start(BytesStart::new("url")))?;
        writer.write_event(Event::Start(BytesStart::new("loc")))?;
        writer.write_event(Event::Text(BytesText::new("http://www.example.com/")))?;
        writer.write_event(Event::End(BytesEnd::new("loc")))?;
        writer.write_event(Event::End(BytesEnd::new("url")))?;

        writer.write_event(Event::End(BytesEnd::new("urlset")))?;
        let result = writer.into_inner().into_inner();
        let expected = concat!(
            r#"<?xml version="1.0" encoding="UTF-8"?>"#,
            r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
            r#"<url>"#,
            r#"<loc>http://www.example.com/</loc>"#,
            r#"</url>"#,
            r#"</urlset>"#
        );
        assert_eq!(result, expected.as_bytes());
        Ok(())
    }
}
