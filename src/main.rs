use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[tokio::main]
async fn main() {
    //A TCP socket server listening to connections which are bound to the specified address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
 
    //handle incoming sockets in a loop
    loop {
        //the second item _ip_addr contains the IP address of the new connection
        let (socket, _ip_addr) = listener.accept().await.unwrap();
        
        //spawn a task for each process
        tokio::spawn(async move {
            process(socket).await;
        });
       
    }

}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    //a hashmap used to store data
    let mut datastore = HashMap::new();

    //connection, provided by `mini_redis` enables parsing frames from the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                datastore.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(data) = datastore.get(cmd.key())  {
                    Frame::Bulk(data.clone().into())
                } else {
                    Frame::Null
                }
            } 
            cmd => panic!("Unimplemented {:?}", cmd),
            
        };

        connection.write_frame(&response).await.unwrap();
    }

    // if let Some(frame) = connection.read_frame().await.unwrap(){
    //     println!("Got: {:?}", frame);

    //     //respond with an error
    //     let response = Frame::Error("Unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }

}