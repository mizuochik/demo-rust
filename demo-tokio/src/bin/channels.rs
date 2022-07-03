use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(64);
    let tx2 = tx.clone();
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".into(),
            resp: resp_tx,
        };
        tx.send(cmd).await.unwrap();
        println!("received {:?}", resp_rx.await);
    });
    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "hello".into(),
            val: "world".into(),
            resp: resp_tx,
        };
        tx2.send(cmd).await.unwrap();
        println!("received {:?}", resp_rx.await);
    });

    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let _ = resp.send(client.get(&key).await);
                }
                Command::Set { key, val, resp } => {
                    let _ = resp.send(client.set(&key, val).await);
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
