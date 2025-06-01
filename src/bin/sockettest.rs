use std::net::SocketAddr;
use socket2::{Socket, Domain, Type, Protocol};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr: SocketAddr = "0.0.0.0:12961".parse().unwrap();

    let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    socket.set_reuse_address(true)?;
    //socket.set_reuse_port(true)?;
    socket.set_broadcast(true)?;
    socket.bind(&addr.into())?;

    let std_socket:std::net::UdpSocket = socket.into();
    std_socket.set_nonblocking(true)?;
    let socket = UdpSocket::from_std(std_socket)?;

    println!("Lausche auf {}", addr);

    let mut buf = [0u8; 1500];
    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        println!("Empfangen von {}: {:?}", src, &buf[..len]);
    }
}
