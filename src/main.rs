use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    //open a connection to a mini-redis address
    let mut client = client::connect("127.0.0.1:8888").await?;

    //set the key: "hello" with the value: "world"
    client.set("hello", "world".into()).await?;

    println!("Hello, world!");

    Ok(())
}
