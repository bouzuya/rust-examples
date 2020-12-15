use actix::prelude::*;

// this is our Message
#[derive(Message)]
#[rtype(result = "usize")] // we have to define the response type for `Sum` message
struct Sum(usize, usize);

// Actor definition
struct Summator;

impl Actor for Summator {
    type Context = Context<Self>;
}

// now we need to define `MessageHandler` for the `Sum` message.
impl Handler<Sum> for Summator {
    type Result = usize; // <- Message response type

    fn handle(&mut self, msg: Sum, _: &mut Context<Self>) -> Self::Result {
        msg.0 + msg.1
    }
}

#[actix_rt::main] // <- starts the system and block until future resolves
async fn main() {
    // -> std::io::Result<()> {
    let addr = Summator.start();
    let res = addr.send(Sum(10, 5)).await; // <- send message and get future for result

    match res {
        Ok(result) => println!("SUM: {}", result),
        _ => println!("Communication to the actor has failed"),
    }
}
