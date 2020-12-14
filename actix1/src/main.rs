fn main() {
    let system = actix::System::new("test");

    system.run();
    // unused `std::result::Result` that must be used
    // `#[warn(unused_must_use)]` on by default
    // this `Result` may be an `Err` variant, which should be handledrustc(unused_must_use)
}
