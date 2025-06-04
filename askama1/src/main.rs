fn main() {
    #[derive(askama::Template)]
    #[template(path = "hello.html")]
    struct HelloTemplate<'a> {
        name: &'a str,
    }

    let hello = HelloTemplate { name: "bouzuya" };
    println!("{}", askama::Template::render(&hello).unwrap());
}
