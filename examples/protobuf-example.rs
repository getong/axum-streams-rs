use axum::response::IntoResponse;
use axum::routing::*;
use axum::Router;
use std::net::SocketAddr;

use futures::prelude::*;
use futures_util::stream::BoxStream;
use tokio_stream::StreamExt;

use axum_streams::*;

#[derive(Clone, prost::Message)]
struct MyTestStructure {
    #[prost(string, tag = "1")]
    some_test_field: String,
}

fn source_test_stream() -> BoxStream<'static, MyTestStructure> {
    // Simulating a stream with a plain vector and throttling to show how it works
    Box::pin(
        stream::iter(vec![
            MyTestStructure {
                some_test_field: "test1".to_string()
            };
            1000
        ])
        .throttle(std::time::Duration::from_millis(50)),
    )
}

async fn test_protobuf_stream() -> impl IntoResponse {
    StreamBodyWith::protobuf(source_test_stream())
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/protobuf-stream", get(test_protobuf_stream));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}