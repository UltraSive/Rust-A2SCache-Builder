use std::error::Error;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UdpSocket;

async fn query_server(address: &str, query_packet: &[u8]) -> Result<(), Box<dyn Error>> {
    let server_socket_addr: SocketAddr = address.to_socket_addrs()?.next().ok_or("Invalid address")?;

    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(server_socket_addr).await?;
    socket.send(query_packet).await?;
    println!("Query: {:?}", query_packet);

    let mut buf: [u8; 4096] = [0u8; 4096];
    let _amt: usize = socket.recv(&mut buf).await?;
    let response: &[u8] = &buf[.._amt];
    let response_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(response);

    if response.len() == 9 {
        println!("Challenge: {}", response_str);

        let challenge: &[u8] = &response[5..9];
        let challenge_packet: Vec<u8> = query_packet
            .iter()
            .chain(challenge.iter())
            .cloned()
            .collect();
        println!("Query Challenge: {:?}", challenge_packet);

        socket.send(&challenge_packet).await?;

        let mut buf_challenge: [u8; 4096] = [0u8; 4096];
        let _amt_challenge: usize = socket.recv(&mut buf_challenge).await?;
        let response_challenge: &[u8] = &buf_challenge[.._amt_challenge];
        let response_str_challenge: std::borrow::Cow<'_, str> =
            String::from_utf8_lossy(response_challenge);
        println!("Response (Challenged): {}", response_str_challenge);
    } else {
        println!("Response: {}", response_str);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server_address: &str = "138.201.130.250:27017";
    let a2s_info: &[u8; 25] = &[
        0xff, 0xff, 0xff, 0xff, b'T', b'S', b'o', b'u', b'r', b'c', b'e', b' ', b'E', b'n', b'g',
        b'i', b'n', b'e', b' ', b'Q', b'u', b'e', b'r', b'y', 0x00,
    ];
    let a2s_player: &[u8; 25] = &[
        0xff, 0xff, 0xff, 0xff, b'T', b'S', b'o', b'u', b'r', b'c', b'e', b' ', b'E', b'n', b'g',
        b'i', b'n', b'e', b' ', b'Q', b'u', b'e', b'r', b'y', 0x00,
    ];

    query_server(server_address, a2s_info).await?;
    query_server(server_address, a2s_player).await?;

    Ok(())
}
