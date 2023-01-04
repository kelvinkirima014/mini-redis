use mini_redis::client;
use tokio::sync::mpsc;
use bytes::Bytes;

enum Commands {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: Bytes,
    }
}

#[tokio::main]
async fn main() {

    let (sender, mut receiver) = mpsc::channel(32);

    let scheduler = tokio::spawn(async move {
        let mut client = client::connect("127..0.0.1:6379").await.unwrap();

        while let Some(cmd) = receiver.recv().await{
            use Commands::*;
            match  cmd  {
                Get { key } => {
                    client.get(&key).await;
                },
                Set { key, value } => {
                    client.set(&key, value).await;
                }
            }
        }

    });

    // let mut client = client::connect("127.0.0.1:6379").await.unwrap();

    // let task1 = tokio::spawn(async {
    //     let res = client.get("foo".into()).await;
    // });

    // let task2 = tokio::spawn(async {
    //     client.set("foo".into(), "bar".into()).await;
    // });

    // task1.await.unwrap();
    // task2.await.unwrap();


}