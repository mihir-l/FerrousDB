use std::net::{Ipv4Addr, SocketAddr};
mod protocol;
use protocol::Request;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::storage::Store;

pub(super) async fn serve(port: u16, store: Store) -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    println!("Listening on: {}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Accepted connection from: {}", addr);
                tokio::spawn(handle_connection(stream, addr, store.clone()));
            }
            Err(e) => {
                eprintln!("ERROR: Failed to accept connection: {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream, addr: SocketAddr, store: Store) {
    let mut read_buffer = [0u8; 1024];
    match stream.read(&mut read_buffer).await {
        Ok(0) => {
            println!("Connection closed by client: {}", addr);
        }
        Ok(n) => {
            let request = Request::from_bytes(&read_buffer[..n]);
            match request {
                Ok(request) => {
                    println!("Received request: {:?}", request);
                    match request.operation {
                        protocol::Operation::Get => match store.get(&request.key).await {
                            Some(value) => {
                                println!("GET {}: {}", request.key, value);
                                let res = protocol::Response {
                                    operation: protocol::Operation::Get,
                                    status: protocol::Status::Ok,
                                    value: Some(value),
                                };
                                stream.write_all(&res.to_bytes()).await.unwrap();
                            }
                            None => {
                                println!("GET {}: Key not found", request.key);
                                let res = protocol::Response {
                                    operation: protocol::Operation::Get,
                                    status: protocol::Status::Error,
                                    value: None,
                                };
                                stream.write_all(&res.to_bytes()).await.unwrap();
                            }
                        },
                        protocol::Operation::Set => {
                            let value = request.value.unwrap();
                            store.set(&request.key, &value).await;
                            println!("SET {}: {:?}", request.key, value);
                            let res = protocol::Response {
                                operation: protocol::Operation::Set,
                                status: protocol::Status::Ok,
                                value: None,
                            };
                            stream.write_all(&res.to_bytes()).await.unwrap();
                        }
                        protocol::Operation::Delete => match store.delete(&request.key).await {
                            Some(value) => {
                                println!("DEL {}: {}", request.key, value);
                                let res = protocol::Response {
                                    operation: protocol::Operation::Delete,
                                    status: protocol::Status::Ok,
                                    value: Some(value),
                                };
                                stream.write_all(&res.to_bytes()).await.unwrap();
                            }
                            None => {
                                println!("DEL {}: Key not found", request.key);
                                let res = protocol::Response {
                                    operation: protocol::Operation::Delete,
                                    status: protocol::Status::Error,
                                    value: None,
                                };
                                stream.write_all(&res.to_bytes()).await.unwrap();
                            }
                        },
                    }
                }
                Err(e) => {
                    eprintln!("ERROR: Failed to parse request: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("ERROR: Failed to read from connection: {}", e);
        }
    }
}
