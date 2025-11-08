fn main() {}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_request() -> anyhow::Result<()> {
        #[derive(Clone)]
        struct MyLayer {
            name: &'static str,
        }

        impl MyLayer {
            fn with_name(name: &'static str) -> Self {
                Self { name }
            }
        }

        impl<S> tower::Layer<S> for MyLayer {
            type Service = MyMiddleware<S>;

            fn layer(&self, inner: S) -> Self::Service {
                MyMiddleware {
                    inner,
                    name: self.name,
                }
            }
        }

        #[derive(Clone)]
        struct MyMiddleware<S> {
            inner: S,
            name: &'static str,
        }

        impl<S> tower::Service<axum::http::Request<axum::body::Body>> for MyMiddleware<S>
        where
            S: tower::Service<
                    axum::http::Request<axum::body::Body>,
                    Response = axum::http::Response<axum::body::Body>,
                > + Send
                + 'static,
            S::Future: Send + 'static,
        {
            type Response = S::Response;
            type Error = S::Error;
            // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
            type Future =
                futures_util::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

            fn poll_ready(
                &mut self,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Result<(), Self::Error>> {
                self.inner.poll_ready(cx)
            }

            fn call(&mut self, mut request: axum::http::Request<axum::body::Body>) -> Self::Future {
                request
                    .headers_mut()
                    .insert("X-NAME", axum::http::HeaderValue::from_static(self.name));

                let future = self.inner.call(request);
                Box::pin(async move {
                    let response: axum::http::Response<axum::body::Body> = future.await?;
                    Ok(response)
                })
            }
        }

        async fn handler(header_map: axum::http::HeaderMap) -> String {
            format!("X-NAME: {:?}", header_map.get("X-NAME"))
        }

        let app = axum::Router::<()>::new()
            .route("/", axum::routing::get(handler))
            .layer(MyLayer::with_name("bouzuya"));
        let response = send_request(
            app,
            axum::http::Request::builder()
                .method("GET")
                .uri("/")
                .body(axum::body::Body::empty())?,
        )
        .await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(
            response.into_body_string().await?,
            "X-NAME: Some(\"bouzuya\")"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_response() -> anyhow::Result<()> {
        #[derive(Clone)]
        struct MyLayer;

        impl<S> tower::Layer<S> for MyLayer {
            type Service = MyMiddleware<S>;

            fn layer(&self, inner: S) -> Self::Service {
                MyMiddleware { inner }
            }
        }

        #[derive(Clone)]
        struct MyMiddleware<S> {
            inner: S,
        }

        impl<S> tower::Service<axum::http::Request<axum::body::Body>> for MyMiddleware<S>
        where
            S: tower::Service<
                    axum::http::Request<axum::body::Body>,
                    Response = axum::http::Response<axum::body::Body>,
                > + Send
                + 'static,
            S::Future: Send + 'static,
        {
            type Response = S::Response;
            type Error = S::Error;
            // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
            type Future =
                futures_util::future::BoxFuture<'static, Result<Self::Response, Self::Error>>;

            fn poll_ready(
                &mut self,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Result<(), Self::Error>> {
                self.inner.poll_ready(cx)
            }

            fn call(&mut self, request: axum::http::Request<axum::body::Body>) -> Self::Future {
                let future = self.inner.call(request);
                Box::pin(async move {
                    let mut response: axum::http::Response<axum::body::Body> = future.await?;
                    *response.body_mut() = axum::body::Body::from("Updated Response Body");
                    Ok(response)
                })
            }
        }

        let app = axum::Router::<()>::new()
            .route("/", axum::routing::get("Original Response"))
            .layer(MyLayer);
        let response = send_request(
            app,
            axum::http::Request::builder()
                .method("GET")
                .uri("/")
                .body(axum::body::Body::empty())?,
        )
        .await?;
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.into_body_string().await?, "Updated Response Body");
        Ok(())
    }

    async fn send_request(
        router: axum::Router<()>,
        request: axum::http::Request<axum::body::Body>,
    ) -> anyhow::Result<axum::response::Response<axum::body::Body>> {
        let response = tower::ServiceExt::oneshot(router, request).await?;
        Ok(response)
    }

    trait ResponseExt {
        async fn into_body_string(self) -> anyhow::Result<String>;
    }

    impl ResponseExt for axum::response::Response<axum::body::Body> {
        async fn into_body_string(self) -> anyhow::Result<String> {
            let bytes = axum::body::to_bytes(self.into_body(), usize::MAX).await?;
            Ok(String::from_utf8(bytes.to_vec())?)
        }
    }
}
