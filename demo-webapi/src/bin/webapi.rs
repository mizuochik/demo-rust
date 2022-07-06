use demo_webapi::di;

#[tokio::main]
async fn main() {
    let di = di::new();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(di.router().into_make_service())
        .await
        .unwrap();
}
