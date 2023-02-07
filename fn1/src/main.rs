fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        fn test_closure<F: Fn() -> i32>(f: F) {
            assert_eq!(3, f());
        }

        let x = 1;
        let f = || x + 2;
        test_closure(f);
        test_closure(f);
        test_closure(f);
        assert_eq!(1, x);
    }

    #[test]
    fn test2() {
        // fn test_closure<F: Fn()>(f: F) {
        //     f();
        // }

        // let x = 1;
        // let f = || x += 2; // コンパイルエラー
        // // expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
        // // this closure implements `FnMut`, not `Fn`
        // test_closure(f);
    }

    #[test]
    fn test3() {
        // fn test_closure<F: Fn() -> String>(f: F) {
        //     assert_eq!("a", f());
        // }

        // let x = String::from("a");
        // let f = move || x; // コンパイルエラー
        // // expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
        // // this closure implements `FnOnce`, not `Fn`
        // test_closure(f);
    }

    #[test]
    fn test4() {
        fn test_closure<F: FnMut()>(mut f: F) {
            f();
        }

        let mut x = 1;
        let f = || x += 2;
        test_closure(f);
        assert_eq!(3, x);
    }

    #[test]
    fn test5() {
        // fn test_closure<F: FnMut()>(mut f: F) {
        //     f();
        // }

        // let mut x = 1;
        // let f = || x += 2;
        // let g = || x += 3; // コンパイルエラー
        // // cannot borrow `x` as mutable more than once at a time
        // // second mutable borrow occurs here
        // test_closure(f);
        // test_closure(g);
    }

    #[test]
    fn test6() {
        fn test_closure<F: FnMut()>(mut f: F) {
            f();
        }

        let mut x = 1;
        let f = || x += 2;
        test_closure(f);

        // NLL : Non-lexical lifetimes
        // <https://blog.rust-lang.org/2022/08/05/nll-by-default.html>
        // <https://rust-lang.github.io/rfcs/2094-nll.html>

        let g = || x += 3;
        test_closure(g);

        assert_eq!(6, x);
    }

    #[test]
    fn test7() {
        // fn test_closure<F: Fn()>(mut f: F) {
        //     f();
        // }

        // let mut x = 1;
        // let f = || x += 2; // コンパイルエラー
        // // expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
        // // this closure implements `FnMut`, not `Fn`
        // test_closure(f);
        // assert_eq!(6, x);
    }

    #[test]
    fn test8() {
        fn test_closure<F: FnMut()>(mut f: F) {
            f();
        }
        let x = 1;
        let f = || println!("{x:?}");
        test_closure(f);
        // assert_eq!(6, x); // おそらく記事の誤り
        assert_eq!(1, x);
    }

    #[test]
    fn test9() {
        // fn test_closure<F: FnMut() -> String>(mut f: F) {
        //     f();
        // }
        // let mut s = String::from("x");
        // let f = move || s; // コンパイルエラー
        // // expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
        // // this closure implements `FnOnce`, not `FnMut`
        // test_closure(f);
    }

    #[test]
    fn test10() {
        // fn test_closure<F: FnMut()>(mut f: F) {
        //     f();
        // }
        // let mut x = 1;
        // let f = || x += 2;
        // test_closure(f);
        // test_closure(f); // コンパイルエラー
        // // use of moved value: `f`
        // // value used here after move
        // assert_eq!(5, x);
    }

    #[test]
    fn test11() {
        fn test_closure<F: FnMut()>(f: &mut F) {
            f();
        }
        let mut x = 1;
        let mut f = || x += 2;
        test_closure(&mut f);
        test_closure(&mut f);
        assert_eq!(5, x);
    }

    #[test]
    fn test12() {
        fn test_closure<F: FnMut()>(mut f: F) {
            f();
        }
        let x = 1;
        let f = || println!("{x:?}");
        test_closure(f);
        test_closure(f);
        // assert_eq!(5, x); // おそらく記事の誤り
        assert_eq!(1, x);
    }

    #[allow(unused_assignments)]
    #[allow(unused_variables)]
    #[test]
    fn test13() {
        fn test_closure<F: FnOnce()>(f: F) {
            f();
        }
        let mut s = String::from("x");
        let f = move || s = "y".to_string();
        test_closure(f);
    }

    #[test]
    fn test14() {
        fn test_closure<F: FnOnce() -> String>(f: F) {
            f();
        }
        let s = String::from("x");
        let f = move || s;
        test_closure(f);
    }

    #[test]
    fn test15() {
        fn test_closure<F: FnOnce()>(f: F) {
            f();
        }
        let s = String::from("x");
        let f = || drop(s);
        test_closure(f);
    }

    #[test]
    fn test16() {
        fn test_closure<F: FnOnce() -> String>(f: F) {
            f();
        }
        let s = String::from("x");
        let f = move || s;
        test_closure(f);
        // assert_eq!("x".to_string(), s); // コンパイルエラー
        // // borrow of moved value: `s`
        // // value borrowed here after move
    }

    #[test]
    fn test17() {
        fn test_closure<F: FnOnce() -> String>(f: F) {
            f();
        }
        let s = String::from("x");
        let f = move || s;
        test_closure(f);
        // test_closure(f); // コンパイルエラー
        // // use of moved value: `f`
        // // value used here after move
    }
}
