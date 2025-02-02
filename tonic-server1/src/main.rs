use hello::greeter_server::{Greeter, GreeterServer};

pub mod hello {
    tonic::include_proto!("hello");
}

pub struct MyGreeter;

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn hello(
        &self,
        request: tonic::Request<hello::HelloRequest>,
    ) -> Result<tonic::Response<hello::HelloResponse>, tonic::Status> {
        let response = hello::HelloResponse {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(tonic::Response::new(response))
    }
}

#[tokio::main]
async fn main() {
    let greeter = MyGreeter;

    tonic::transport::Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve("0.0.0.0:3000".parse().unwrap())
        .await
        .unwrap();
}
