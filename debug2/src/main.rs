struct Tuple(i32, String);

impl std::fmt::Debug for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Tuple")
            .field(&self.0)
            .field(&self.1)
            .finish()
    }
}

struct Struct {
    b: bool,
    m: std::collections::BTreeMap<i32, String>,
    s: std::collections::BTreeSet<i32>,
    t: (i32, String),
    v: Vec<i32>,
}

impl std::fmt::Debug for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Struct")
            .field("b", &self.b)
            .field("m", &self.m)
            .field("s", &self.s)
            .field("t", &self.t)
            .field("v", &self.v)
            .finish()
    }
}

struct MyVec(i32, i32);

impl std::fmt::Debug for MyVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entry(&self.0).entry(&self.1).finish()
    }
}

struct MyMap(i32, i32);

impl std::fmt::Debug for MyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entry(&self.0, &self.1).finish()
    }
}

struct MySet(i32, i32);

impl std::fmt::Debug for MySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_set().entry(&self.0).entry(&self.1).finish()
    }
}

fn main() {
    assert_eq!(
        format!("{:?}", Tuple(1, "abc".to_owned())),
        "Tuple(1, \"abc\")"
    );
    assert_eq!(
        format!(
            "{:?}",
            Struct {
                b: true,
                m: [(1, "abc".to_owned()), (2, "def".to_owned())]
                    .into_iter()
                    .collect::<std::collections::BTreeMap<i32, String>>(),
                s: [1, 2, 3]
                    .into_iter()
                    .collect::<std::collections::BTreeSet<i32>>(),
                t: (1, "abc".to_owned()),
                v: vec![1, 2, 3]
            }
        ),
        "Struct { b: true, m: {1: \"abc\", 2: \"def\"}, s: {1, 2, 3}, t: (1, \"abc\"), v: [1, 2, 3] }"
    );

    assert_eq!(format!("{:?}", MyVec(1, 2)), "[1, 2]");
    assert_eq!(format!("{:?}", MyMap(1, 2)), "{1: 2}");
    assert_eq!(format!("{:?}", MySet(1, 2)), "{1, 2}");
}
