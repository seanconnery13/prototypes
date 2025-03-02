use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use rand::Rng;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    let targets = get_targets();

    let listener = TcpListener::bind("127.0.0.1:8085").await.unwrap();
    println!("L4 Load balancer listening on 8085");
    loop {
        let clone = Arc::clone(&targets);
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut rng = rand::rng();
        let index = rng.random_range(0..targets.len());
        tokio::spawn(async move {
            process(socket, clone, index).await;
        });
    }
}

async fn process(mut socket: TcpStream, targets: Arc<Vec<IpPort>>, index: usize) {
    let target = targets.get(index).unwrap();
    let target_address =  format!("{}:{}", target.ip.to_string(), target.port.to_string());
    println!("{}", targetAddress);

    let mut stream = TcpStream::connect(targetAddress).await.unwrap();
    let mut resp = String::from("");
    stream.read_to_string(&mut resp).await.unwrap();
    socket.write_all(resp.as_bytes()).await.unwrap();
}

fn get_targets() -> Arc<Vec<IpPort>> {
    let mut arr = vec![];
    let server1 = IpPort {
        ip : IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port : 8080,
    };
    let server2 = IpPort {
        ip : IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port : 8081,
    };
    let server3 = IpPort {
        ip : IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port : 8082,
    };
    let server4 = IpPort {
        ip : IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        port : 8083,
    };

    arr.push(server1);
    arr.push(server2);
    arr.push(server3);
    arr.push(server4);

    Arc::new(arr)
}

pub struct IpPort{
   ip: IpAddr, port: u16,
}
