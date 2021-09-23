struct MyIter {
    i: usize,
    v: MyVec,
}

impl Iterator for MyIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        if i < self.v.0.len() {
            self.i += 1;
            Some(self.v.0[i])
        } else {
            None
        }
    }
}

struct MyVec(Vec<usize>);

impl IntoIterator for MyVec {
    type Item = usize;

    type IntoIter = MyIter;

    fn into_iter(self) -> Self::IntoIter {
        MyIter { i: 0, v: self }
    }
}

#[test]
fn test() {
    let v = MyVec(vec![123, 456, 789]);
    assert_eq!(v.into_iter().collect::<Vec<usize>>(), vec![123, 456, 789]);
}
