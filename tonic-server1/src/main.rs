use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod hello {
    tonic::include_proto!("hello");
}

#[derive(Debug)]
pub struct MyGreeter;

#[tonic::async_trait]
impl hello::greeter_server::Greeter for MyGreeter {
    #[tracing::instrument]
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

#[derive(Debug)]
pub struct MyService2;

#[tonic::async_trait]
impl hello::service2_server::Service2 for MyService2 {
    #[tracing::instrument]
    async fn check_metadata(
        &self,
        request: tonic::Request<hello::CheckMetadataRequest>,
    ) -> Result<tonic::Response<hello::CheckMetadataResponse>, tonic::Status> {
        let bouzuya_id = request
            .metadata()
            .get("x-bouzuya-id")
            .ok_or_else(|| tonic::Status::unauthenticated("no x-bouzuya-id"))?
            .to_str()
            .map_err(|_| tonic::Status::unauthenticated("x-bouzuya-id is not str"))?
            .to_owned();
        Ok(tonic::Response::new(hello::CheckMetadataResponse {
            bouzuya_id,
        }))
    }

    #[tracing::instrument]
    async fn error(
        &self,
        _request: tonic::Request<hello::ErrorRequest>,
    ) -> Result<tonic::Response<hello::ErrorResponse>, tonic::Status> {
        // Ok(tonic::Response::new(hello::ErrorResponse {}))
        // Err(tonic::Status::unimplemented("my unimplemented message"))
        Err(tonic::Status::unauthenticated("my unauthenticated message"))
    }

    #[tracing::instrument]
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
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().with_span_events(
            tracing_subscriber::fmt::format::FmtSpan::NEW
                | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
        ))
        .init();

    tonic::transport::Server::builder()
        .trace_fn(|_http_request| tracing::info_span!("info_span"))
        .add_service(hello::greeter_server::GreeterServer::new(MyGreeter))
        .add_service(hello::service2_server::Service2Server::new(MyService2))
        .serve("0.0.0.0:3000".parse().unwrap())
        .await
        .unwrap();
}
