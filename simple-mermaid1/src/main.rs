mod model;

fn main() {
    let created = model::Aggregate1::new();
    let updated = created.update();
    println!("{}", updated.state());
}
