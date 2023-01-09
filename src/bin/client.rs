use bytes::Bytes;
use mini_redis::client;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Commands {
    Get { key: String },
    Set { key: String, value: Bytes },
}

#[tokio::main]
async fn main() {
    let (sender, mut receiver) = mpsc::channel(32);

    //clone the sender channel so we can have multiple senders
    let another_sender = sender.clone();

    //spawn a task that processes messages from the channel
    let task_message_processor = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = receiver.recv().await {
            use Commands::*;
            match cmd {
                Get { key } => {
                    client.get(&key).await.unwrap();
                }
                Set { key, value } => {
                    client.set(&key, value).await.unwrap();
                }
            }
        }
    });

    //clone sender channel

    //task to get commands over a channel
    let task_get_command = tokio::spawn(async move {
        let cmd = Commands::Get {
            key: "foo".to_string(),
        };

        sender.send(cmd).await.unwrap();
    });

    //task to set commands over another sender channel
    let task_set_command = tokio::spawn(async move {
        let cmd = Commands::Set {
            key: "foo".to_string(),
            value: "bar".into(),
        };

        another_sender.send(cmd).await.unwrap();
    });

    task_get_command.await.unwrap();
    task_set_command.await.unwrap();
    task_message_processor.await.unwrap();
}
