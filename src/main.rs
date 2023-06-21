use std::error::Error;
use std::net::{ SocketAddr, ToSocketAddrs };
use tokio::io::{ AsyncReadExt, AsyncWriteExt };
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Constants
    let server_address: &str = "127.0.0.1:28017";
    let server_socket_addr: SocketAddr = server_address
        .to_socket_addrs()?
        .next()
        .ok_or("Invalid address")?;

    // A2S_INFO
    let query_packet: &[u8; 25] = &[
        0xff,
        0xff,
        0xff,
        0xff,
        b'T',
        b'S',
        b'o',
        b'u',
        b'r',
        b'c',
        b'e',
        b' ',
        b'E',
        b'n',
        b'g',
        b'i',
        b'n',
        b'e',
        b' ',
        b'Q',
        b'u',
        b'e',
        b'r',
        b'y',
        0x00,
    ];

    // Allocating socket
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").await?;
    socket.connect(server_socket_addr).await?;
    socket.send(query_packet).await?;
    println!("Query: {:?}", query_packet);

    // Allocating memory space for query and sending
    let mut buf: [u8; 4096] = [0u8; 4096];
    let _amt: usize = socket.recv(&mut buf).await?;
    let response: &[u8] = &buf[.._amt];
    let response_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(response);

    if response.len() == 9 {
        // If its a challenge instead of actual data response
        println!("Challenge: {}", response_str);
        // Allocating memory space for query and sending query again with last 4 bits of challenge
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
        let response_str_challenge: std::borrow::Cow<'_, str> = String::from_utf8_lossy(
            response_challenge
        );
        println!("Response (Challenged): {}", response_str_challenge);
    } else {
        println!("Response: {}", response_str);
    }

    Ok(())
}
