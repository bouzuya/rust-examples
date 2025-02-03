pub mod hello {
    tonic::include_proto!("hello");
}

pub struct MyGreeter;

#[tonic::async_trait]
impl hello::greeter_server::Greeter for MyGreeter {
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

pub struct MyService2;

#[tonic::async_trait]
impl hello::service2_server::Service2 for MyService2 {
    async fn scalar_value_type(
        &self,
        request: tonic::Request<hello::ScalarValueTypeRequest>,
    ) -> Result<tonic::Response<hello::ScalarValueTypeResponse>, tonic::Status> {
        let hello::ScalarValueTypeRequest {
            double_value,
            float_value,
            int32_value,
            int64_value,
            uint32_value,
            uint64_value,
            sint32_value,
            sint64_value,
            fixed32_value,
            fixed64_value,
            sfixed32_value,
            sfixed64_value,
            bool_value,
            string_value,
            bytes_value,
        } = request.into_inner();
        let response = hello::ScalarValueTypeResponse {
            double_value,
            float_value,
            int32_value,
            int64_value,
            uint32_value,
            uint64_value,
            sint32_value,
            sint64_value,
            fixed32_value,
            fixed64_value,
            sfixed32_value,
            sfixed64_value,
            bool_value,
            string_value,
            bytes_value,
        };
        Ok(tonic::Response::new(response))
    }
}
#[tokio::main]
async fn main() {
    tonic::transport::Server::builder()
        .add_service(hello::greeter_server::GreeterServer::new(MyGreeter))
        .add_service(hello::service2_server::Service2Server::new(MyService2))
        .serve("0.0.0.0:3000".parse().unwrap())
        .await
        .unwrap();
}
