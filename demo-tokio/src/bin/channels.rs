use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(64);
    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        tx.send(Command::Get { key: "first".into() }).await;
    });
    let t2 = tokio::spawn(async move {
        tx2.send(Command::Set { key: "vv".into(), val: "second".into() }).await;
    });

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key } => {
                    client.get(&key).await;
                }
                Command::Set { key, val } => {
                    client.set(&key, val).await;
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
