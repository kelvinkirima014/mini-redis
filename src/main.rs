use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
 
    //hadle incoming sockets in a loop
    loop {
        //the second item _ contains the IP and port of the new connection
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }

}

async fn process(socket: TcpStream) {
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap(){
        println!("Got: {:?}", frame);

        //respond with an error
        let response = Frame::Error("Unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }

}