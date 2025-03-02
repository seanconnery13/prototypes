use tokio::io::{AsyncWriteExt};
use tokio::net::{TcpListener};
#[tokio::main]
async fn main() {
        tokio::spawn(async move {
            server1().await;
        });
        tokio::spawn(async move {
            server2().await;
        });
        tokio::spawn(async move {
            server3().await;
        });
        tokio::spawn(async move {
            server4().await;
        });
    loop {
        tokio::task::yield_now().await;
    }
}

async fn server1() {
    let listener1 = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("a server listening on 8080");
    loop {
        let (mut socket1, _) = listener1.accept().await.unwrap();
        socket1.write_all("first server ack".as_bytes()).await.unwrap();
    }
}
async fn server2() {
    let listener1 = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    println!("a server listening on 8081");
    loop {
        let (mut socket1, _) = listener1.accept().await.unwrap();
        socket1.write_all("second server ack".as_bytes()).await.unwrap();
    }
}
async fn server3() {
    let listener1 = TcpListener::bind("127.0.0.1:8082").await.unwrap();
    println!("a server listening on 8083");
    loop {
        let (mut socket1, _) = listener1.accept().await.unwrap();
        socket1.write_all("third server ack".as_bytes()).await.unwrap();
    }
}
async fn server4() {
    let listener1 = TcpListener::bind("127.0.0.1:8083").await.unwrap();
    println!("a server listening on 8084");
    loop {
        let (mut socket1, _) = listener1.accept().await.unwrap();
        socket1.write_all("fourth server ack".as_bytes()).await.unwrap();
    }
}
