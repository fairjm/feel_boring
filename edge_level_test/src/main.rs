use mio::net::{TcpListener, TcpStream};
use mio::*;
use std::collections::HashMap;
use std::env;
use std::io::prelude::*;

use std::io;
use std::io::Result;

const SERVER: Token = Token(0);

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let read_opts = if args.len() > 1 && args[1] == "-e" {
        println!("use edge");
        PollOpt::edge()
    } else {
        println!("default is level. use -e for edge");
        PollOpt::level()
    };
    let poll = Poll::new()?;
    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr)?;
    let mut map = HashMap::new();
    poll.register(&listener, SERVER, Ready::readable(), PollOpt::level())?;

    let mut unique_token = Token(SERVER.0 + 1);

    let mut events = Events::with_capacity(1024);
    loop {
        println!("=================");
        println!("another loop");
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            println!("event : {:?}", event);
            match event.token() {
                SERVER => {
                    let (stream, addr) = listener.accept()?;
                    println!("======new link======");
                    println!("from:{:?}", addr);
                    let next_token = next(&mut unique_token);
                    poll.register(&stream, next_token, Ready::readable(), read_opts)?;
                    map.insert(next_token, stream);
                }
                token => {
                    if let Some(stream) = map.get_mut(&token) {
                        if event.readiness().is_readable() {
                            if read_opts.is_level() {
                                read_level(stream, &poll)?;
                            } else if read_opts.is_edge() {
                                read_edge(stream)?;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn read_level(stream: &mut TcpStream, poll: &Poll) -> Result<()> {
    let mut connection_closed = false;
    loop {
        let mut buf = vec![0u8; 8];
        match stream.read(&mut buf) {
            Ok(0) => {
                connection_closed = true;
                break;
            }
            Ok(_n) => {
                println!("======come new read======");
                println!("read:{:?}", String::from_utf8(buf));
                // just return is ok for level
                return Ok(());
            }
            Err(ref err) if would_block(err) => {
                println!("would_block happened");
                break;
            }
            Err(ref err) if interrupted(err) => {
                println!("interrupted happened");
                continue;
            }
            Err(err) => return Err(err),
        }
    }
    if connection_closed {
        // must have this one
        poll.deregister(stream)?;
        println!("{:?} Connection closed", stream.peer_addr());
    }
    Ok(())
}

fn read_edge(stream: &mut TcpStream) -> Result<()> {
    let mut connection_closed = false;
    loop {
        let mut buf = vec![0u8; 8];
        match stream.read(&mut buf) {
            Ok(0) => {
                connection_closed = true;
                break;
            }
            Ok(_n) => {
                println!("======come new read======");
                println!("read:{:?}", String::from_utf8(buf));
            }
            Err(ref err) if would_block(err) => {
                println!("would_block happened");
                // edge rely this to return, without this or just return after read(like level)
                // the connection will not be read anymore
                break;
            }
            Err(ref err) if interrupted(err) => {
                println!("interrupted happened");
                continue;
            }
            // Other errors we'll consider fatal.
            Err(err) => return Err(err),
        }
    }
    if connection_closed {
        println!("{:?} Connection closed", stream.peer_addr());
    }
    Ok(())
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
