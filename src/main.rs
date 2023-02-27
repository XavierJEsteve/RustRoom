use std::net::SocketAddr;

use::tokio::{
    net::TcpListener, 
    sync::broadcast,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader}
};

#[derive(Clone, Debug)]
struct  Message {
    text: String,
    contact: SocketAddr
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, rx) = broadcast::channel::<Message>(10);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
    
        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn( async move {
            let (reader, mut writer) = socket.split();
    
            let mut reader = BufReader::new(reader);    
            let mut line  = String::new();
            
            loop {
                tokio::select! {
                    result = rx.recv() => {
                        let msg = result.unwrap(); // msg should be of Enum 'Message'
                        if addr != msg.contact { // Listener accepts a SocketAddr (addr) when the connection is established.
                            writer.write_all(&msg.text.as_bytes()).await.unwrap(); //  Only use the writer for msgs from new addrresses and save 
                        }
                    }
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        
                        tx.send(Message { text: (line.clone()), contact: (addr) }).unwrap();
                        line.clear();
                    }
                }
            }
        });
    }
}
