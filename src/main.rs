use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::sync::{Arc, Mutex};
use bytes::Bytes;
use std::collections::HashMap;


type Db = Arc<Mutex<HashMap<String,Bytes>>>;


#[tokio::main]
async fn main() {
    let mut listener = TcpListener::bind("127.0.0.1:30006").await.unwrap();

    println!("listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone;

        println!("accepted");
        process(socket, db).await;
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};


    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd)
        };

        connection.write_frame(&response).await.unwrap();
    }
}