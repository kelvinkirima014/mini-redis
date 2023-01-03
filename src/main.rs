//Start a server and use `tokio::spawn` to start a new task that processes each received connection.
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Datastore = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    //A TCP socket server listening to connections which are bound to the specified address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening to connection...");

    let datastore = Arc::new(Mutex::new(HashMap::new()));
 
    //handle incoming sockets in a loop
    loop {
        //the second item _ip_addr contains the IP address of the new connection
        let (socket, _ip_addr) = listener.accept().await.unwrap();
        
        let datastore = datastore.clone();


        println!("Connection accepted");
        //spawn a task for each process
       
        tokio::spawn(async move {
            process(socket, datastore).await;
        });
       
    }

}

async fn process(socket: TcpStream, datastore: Datastore) {
    use mini_redis::Command::{self, Get, Set};
    //connection, provided by `mini_redis` enables parsing frames from the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap(){
        let response = match Command::from_frame(frame).unwrap(){
            Set(cmd) => {
                let mut datastore = datastore.lock().unwrap();
                datastore.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let datastore = datastore.lock().unwrap();
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