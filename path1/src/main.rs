// std::path の例
//
// TODO: Path
// TODO: PathBuf
// TODO: PrefixComponent
// TODO: StripPrefixError
// TODO: Component
// TODO: Prefix
//
// <https://doc.rust-lang.org/std/path/index.html>
//
// Structs
// - Ancestors ... Path の ancestors() で作られる struct
// - Components ... Path の components() で作られる struct 。 Path の Component の Iterator 。
// - Display ... Path の display() で作られる struct 。 Display のためのヘルパ。
// - Iter ... Path の iter() で作られる struct 。 Path の Component の OsStr の Iterator 。
// - Path ... Path 。 str 相当。
// - PathBuf ... PathBuf 。 String 相当。
// - PrefixComponent
// - StripPrefixError
//
// Enums
// - Component
// - Prefix
//
// Constants
// - MAIN_SEPARATOR
//
// Functions
// - is_separator
#[cfg(test)]
mod tests {
    #[test]
    fn ancestors_tests() {
        use std::path::{Ancestors, Path};
        let mut ancestors: Ancestors = Path::new("/foo/bar").ancestors();
        assert_eq!(ancestors.next(), Some(Path::new("/foo/bar")));
        assert_eq!(ancestors.next(), Some(Path::new("/foo")));
        assert_eq!(ancestors.next(), Some(Path::new("/")));
        assert_eq!(ancestors.next(), None);
    }

    #[test]
    fn components_iterator_tests() {
        use std::{
            ffi::OsStr,
            path::{Component, Components, Path},
        };
        let path: &Path = Path::new("/tmp/foo/bar.txt");
        let mut components: Components = path.components();
        assert_eq!(components.next(), Some(Component::RootDir));
        assert_eq!(
            components.next(),
            Some(Component::Normal(&OsStr::new("tmp")))
        );
        assert_eq!(
            components.next(),
            Some(Component::Normal(&OsStr::new("foo")))
        );
        assert_eq!(
            components.next(),
            Some(Component::Normal(&OsStr::new("bar.txt")))
        );
        assert_eq!(components.next(), None);
    }

    #[test]
    fn components_as_path_tests() {
        use std::path::{Components, Path};
        let path: &Path = Path::new("/tmp/foo/bar.txt");
        let components: Components = path.components();
        assert_eq!(components.as_path(), path);
    }

    #[test]
    fn display_tests() {
        use std::path::Path;
        let path = Path::new("/tmp/foo.rs");
        assert_eq!("/tmp/foo.rs", path.display().to_string());
        // x.to_string() => format!("{}", x);
    }

    #[test]
    fn iter_next_tests() {
        use std::{
            ffi::OsStr,
            path::{Path, MAIN_SEPARATOR},
        };
        let path = Path::new("/tmp/foo/bar.rs");
        let mut iter = path.iter();
        assert_eq!(iter.next(), Some(OsStr::new(&MAIN_SEPARATOR.to_string())));
        assert_eq!(iter.next(), Some(OsStr::new("tmp")));
        assert_eq!(iter.next(), Some(OsStr::new("foo")));
        assert_eq!(iter.next(), Some(OsStr::new("bar.rs")));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_as_path_tests() {
        use std::path::Path;
        let path = Path::new("/tmp/foo/bar.rs");
        assert_eq!(path, path.iter().as_path());
    }

    #[test]
    fn path_new_tests() {
        use std::{ffi::OsStr, path::Path};
        let path: &Path = Path::new("/tmp/foo.rs");
        assert_eq!(path.as_os_str(), OsStr::new("/tmp/foo.rs"));
    }

    #[test]
    fn path_as_os_str_tests() {
        use std::{ffi::OsStr, path::Path};
        let s: &OsStr = Path::new("/tmp/foo.rs").as_os_str();
        assert_eq!(s, OsStr::new("/tmp/foo.rs"));
    }

    #[test]
    fn path_to_str_tests() {
        // UTF-8 として valid なとき Some になるらしい
        use std::path::Path;
        let s: Option<&str> = Path::new("/tmp/foo.rs").to_str();
        assert_eq!(s, Some("/tmp/foo.rs"));
    }

    #[test]
    fn path_to_string_lossy_tests() {
        // UTF-8 として valid でないなど U+FFFD に置き換えられるらしい
        use std::{borrow::Cow, path::Path};
        let s: Cow<str> = Path::new("/tmp/foo.rs").to_string_lossy();
        assert_eq!(s, "/tmp/foo.rs");
    }

    #[test]
    fn path_to_path_buf_tests() {
        use std::path::{Path, PathBuf};
        // &Path -> PathBuf なので &str -> String 的なもの
        let s: PathBuf = Path::new("/tmp/foo.rs").to_path_buf();
        assert_eq!(s, PathBuf::from("/tmp/foo.rs"));
    }

    #[test]
    fn path_is_absolute_tests() {
        // Unix 環境では Path の has_root と変わらないらしい
        use std::path::Path;
        let b1: bool = Path::new("/tmp/foo.rs").is_absolute();
        assert_eq!(b1, true);
        let b2: bool = Path::new("./tmp/foo.rs").is_absolute();
        assert_eq!(b2, false);
    }

    #[test]
    fn path_is_relative_tests() {
        use std::path::Path;
        let b1: bool = Path::new("/tmp/foo.rs").is_relative();
        assert_eq!(b1, false);
        let b2: bool = Path::new("./tmp/foo.rs").is_relative();
        assert_eq!(b2, true);
    }

    #[test]
    fn path_has_root_tests() {
        // Unix 環境では Path の is_absolute と変わらないらしい
        use std::path::Path;
        let b1: bool = Path::new("/tmp/foo.rs").has_root();
        assert_eq!(b1, true);
        let b2: bool = Path::new("./tmp/foo.rs").has_root();
        assert_eq!(b2, false);
    }

    #[test]
    fn path_parent_tests() {
        use std::path::Path;
        // root の場合に None
        let p1: Option<&Path> = Path::new("/tmp/foo/bar.rs").parent();
        assert_eq!(p1, Some(Path::new("/tmp/foo")));
        let p2: Option<&Path> = p1.unwrap().parent();
        assert_eq!(p2, Some(Path::new("/tmp")));
        let p3: Option<&Path> = p2.unwrap().parent();
        assert_eq!(p3, Some(Path::new("/")));
        let p4: Option<&Path> = p3.unwrap().parent();
        assert_eq!(p4, None);
    }

    #[test]
    fn path_ancestors_tests() {
        // ancestors_tests を参照
    }

    // TODO: next: <https://doc.rust-lang.org/std/path/struct.Path.html#method.file_name>

    #[test]
    fn path_join_tests() {
        // Path の push は PathBuf の push と同じもの
        // もし絶対パスの場合はカレントパスが置き換えられる
        use std::path::Path;
        let p1 = Path::new("/tmp/foo.rs");
        let p2 = Path::new("./tmp/foo.rs");
        assert_eq!(p1.join(p1), Path::new("/tmp/foo.rs"));
        assert_eq!(p1.join(p2), Path::new("/tmp/foo.rs/./tmp/foo.rs"));
    }
}

fn main() {
    println!("Hello, world!");
}
