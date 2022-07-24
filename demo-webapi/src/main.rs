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
    let mut interrupt = unix::signal(unix::SignalKind::interrupt()).unwrap();
    let mut terminate = unix::signal(unix::SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = interrupt.recv() => {},
        _ = terminate.recv() => {},
    }
    println!("Signal received");
}
