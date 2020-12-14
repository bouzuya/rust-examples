use actix::{Actor, Context, Running, System};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("I am alive!");
        System::current().stop(); // <- stop system
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        println!("stopping");
        Running::Stop
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        println!("stopped");
    }
}

fn main() {
    let system = System::new("test");

    let _ = MyActor.start();

    system.run().expect("error");
}
