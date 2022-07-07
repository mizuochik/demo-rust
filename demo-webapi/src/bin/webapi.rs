use demo_webapi::di;
use tokio::signal::unix;

#[tokio::main]
async fn main() {
    let di = di::new();
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(di.router().into_make_service())
        .with_graceful_shutdown(shutdown())
        .await
        .unwrap();
}

async fn shutdown() {
    let interrupt = async {
        unix::signal(unix::SignalKind::interrupt())
            .unwrap()
            .recv()
            .await;
    };
    let terminate = async {
        unix::signal(unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await;
    };
    tokio::select! {
        _ = interrupt => {},
        _ = terminate => {},
    }
    println!("Signal received");
}
