pub fn wrap<F: Fn(char) -> f32>(s: &str, max_width: f32, get_width: F) -> String {
    // TODO: ...
    let chars = ",.、。々〉》」』】〕〗〙〜〟〻";
    let mut x = 0_f32;
    let mut t = String::new();
    for c in s.chars() {
        if c == '\n' {
            x = 0_f32;
            t.push(c);
        } else {
            let w = get_width(c);
            if x + w > max_width {
                x = 0_f32;
                // 禁則処理
                if chars.contains(c) {
                    match t.pop() {
                        Some(pc) => {
                            if pc == '\n' || chars.contains(pc) {
                                t.push(pc);
                                t.push('\n');
                            } else {
                                t.push('\n');
                                t.push(pc);
                                x = get_width(pc);
                            }
                        }
                        None => {
                            t.push('\n');
                        }
                    }
                } else {
                    t.push('\n');
                }
            }
            t.push(c);
            x += w;
        }
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(wrap("", 3.0, |_| 1.0), "");
        assert_eq!(wrap("a", 3.0, |_| 1.0), "a");
        assert_eq!(wrap("ab", 3.0, |_| 1.0), "ab");
        assert_eq!(wrap("abc", 3.0, |_| 1.0), "abc");
        assert_eq!(wrap("abcd", 3.0, |_| 1.0), "abc\nd");
        assert_eq!(wrap("abcdefg", 3.0, |_| 1.0), "abc\ndef\ng");
    }

    #[test]
    fn test_lf() {
        assert_eq!(wrap("\n", 3.0, |_| 1.0), "\n");
        assert_eq!(wrap("abc\n", 3.0, |_| 1.0), "abc\n");
        assert_eq!(wrap("abc\ndef", 3.0, |_| 1.0), "abc\ndef");
        assert_eq!(wrap("ab\ncdef", 3.0, |_| 1.0), "ab\ncde\nf");
        assert_eq!(wrap("ab\ncd\nefgh", 3.0, |_| 1.0), "ab\ncd\nefg\nh");
    }

    #[test]
    fn test_japanese_hyphenation() {
        assert_eq!(wrap(".", 3.0, |_| 1.0), ".");
        assert_eq!(wrap("ab.", 3.0, |_| 1.0), "ab.");
        assert_eq!(wrap("abc.", 3.0, |_| 1.0), "ab\nc."); // != "abc\n."
        assert_eq!(wrap("abc.def", 3.0, |_| 1.0), "ab\nc.d\nef");

        // U+3001 '、'
        assert_eq!(wrap("abc、", 3.0, |_| 1.0), "ab\nc、");
        // U+3002 '。'
        assert_eq!(wrap("abc。", 3.0, |_| 1.0), "ab\nc。");
        // U+3005 '々'
        assert_eq!(wrap("abc々", 3.0, |_| 1.0), "ab\nc々");
        // U+3009 '〉'
        assert_eq!(wrap("abc〉", 3.0, |_| 1.0), "ab\nc〉");
        // U+300B '》'
        assert_eq!(wrap("abc》", 3.0, |_| 1.0), "ab\nc》");
        // U+300D '」'
        assert_eq!(wrap("abc」", 3.0, |_| 1.0), "ab\nc」");
        // U+300F '』'
        assert_eq!(wrap("abc』", 3.0, |_| 1.0), "ab\nc』");
        // U+3011 '】'
        assert_eq!(wrap("abc】", 3.0, |_| 1.0), "ab\nc】");
        // U+3015 '〕'
        assert_eq!(wrap("abc〕", 3.0, |_| 1.0), "ab\nc〕");
        // U+3017 '〗'
        assert_eq!(wrap("abc〗", 3.0, |_| 1.0), "ab\nc〗");
        // U+3019 '〙'
        assert_eq!(wrap("abc〙", 3.0, |_| 1.0), "ab\nc〙");
        // (!) U+301B '〛'
        assert_eq!(wrap("abc〛", 3.0, |_| 1.0), "abc\n〛");
        // U+301C '〜'
        assert_eq!(wrap("abc〜", 3.0, |_| 1.0), "ab\nc〜");
        // U+301F '〟'
        assert_eq!(wrap("abc〟", 3.0, |_| 1.0), "ab\nc〟");
        // U+303B '〻'
        assert_eq!(wrap("abc〻", 3.0, |_| 1.0), "ab\nc〻");

        assert_eq!(wrap("abc,", 3.0, |_| 1.0), "ab\nc,");
    }

    #[test]
    fn test() {
        assert_eq!(wrap("", 3.0, |_| 1.0), "");
        assert_eq!(wrap("a", 3.0, |_| 1.0), "a");
        assert_eq!(wrap("ab", 3.0, |_| 1.0), "ab");
        assert_eq!(wrap("abc", 3.0, |_| 1.0), "abc");
        assert_eq!(wrap("abcd", 3.0, |_| 1.0), "abc\nd");
        assert_eq!(wrap("abcdef", 3.0, |_| 1.0), "abc\ndef");
        assert_eq!(wrap("abcdefg", 3.0, |_| 1.0), "abc\ndef\ng");
        assert_eq!(wrap(".", 3.0, |_| 1.0), ".");
        assert_eq!(wrap("ab.", 3.0, |_| 1.0), "ab.");
        assert_eq!(wrap("ab\n.", 3.0, |_| 1.0), "ab\n.");
        assert_eq!(wrap("ab.cdef", 3.0, |_| 1.0), "ab.\ncde\nf");
        assert_eq!(wrap("abc.", 3.0, |_| 1.0), "ab\nc.");
        assert_eq!(wrap("abc,", 3.0, |_| 1.0), "ab\nc,");
        assert_eq!(wrap("abc、", 3.0, |_| 1.0), "ab\nc、");
        assert_eq!(wrap("abc。", 3.0, |_| 1.0), "ab\nc。");
        assert_eq!(wrap("abc.def", 3.0, |_| 1.0), "ab\nc.d\nef");
    }
}
