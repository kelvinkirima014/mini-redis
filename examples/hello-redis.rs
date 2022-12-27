use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    //Initialize a min-redis client by openning a connection to a mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    //set the key: "hello" with the value: "world"
    client.set("hello", "world".into()).await?;

    //get the key "hello"
    let result = client.get("hello").await?;
    
    println!("Got the value from the server. Result = {:?}", result);

    Ok(())
}
