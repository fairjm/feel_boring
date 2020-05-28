use std::env;

use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;
use futures::future::try_join;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8999".to_string());
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = [0; 2048];
            let n = socket.read(&mut buf).await.expect("failed to read data from socket");
            if n == 0 {
                println!("disconnect");
                return;
            }
            let content = String::from_utf8_lossy(&buf).to_string();
            println!("content {}", content);
            if let Some(host) = parse_connect(&content) {
                println!("host: {}", host);
                let mut server_socket = TcpStream::connect(&host).await.unwrap();
                socket
                    .write_all("HTTP/1.1 200 Connection established\r\n\r\n".as_bytes())
                    .await
                    .unwrap();
                loop {
                    println!("loop~~");
                    let (mut socket_in, mut socket_out) = socket.split();
                    let (mut server_in, mut server_out) = server_socket.split();
                    let client_to_server = io::copy(&mut socket_in, &mut server_out);
                    let server_to_client = io::copy(&mut server_in, &mut socket_out);
                    match try_join(client_to_server, server_to_client).await {
                        Err(e) => {
                            println!("exits. Err: {:?}", e);
                            break; },
                        Ok((cs, sc)) => {
                            if cs ==0 || sc==0 {
                                println!("connection return zero");
                                break;
                            }
                        }


                    }
                }
            } else {
                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
                return;
            }
        });
    }
}

fn parse_connect(content: &str) -> Option<String> {
    let lines: Vec<&str> = content.split("\r\n").collect();
    if lines.is_empty() {
        return None;
    }
    let line = lines.get(0).unwrap();
    let elems: Vec<&str> = line.split(" ").collect();
    if elems.len() < 2
        || !"connect".eq_ignore_ascii_case(elems.get(0).unwrap_or(&"")) {
        return None;
    }
    return Some(String::from(*elems.get(1).unwrap()));
}
