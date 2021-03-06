use std::net::TcpListener;
use std::io::prelude::*;
use std::fs;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();
    let args:Vec<String> = std::env::args().collect();
    let thread_num = if args.len() < 2 {
        4
    } else {
        args[1].parse().unwrap()
    };
    let pool = ThreadPool::new(thread_num);
    for stream in listener.incoming() {
        pool.execute( || handle_connection(stream.unwrap()));
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream : std::net::TcpStream) {
    let mut http_req= [0 as u8;512];
    stream.read(&mut http_req).unwrap();
    let content = String::from_utf8_lossy(&http_req);
    if content.starts_with("GET / HTTP/1.1\r\n") {
        send_response("HTTP/1.1 200 OK", "hello.html", &mut stream);
    } else if content.starts_with("GET /sleep HTTP/1.1\r\n") {
        thread::sleep(Duration::from_secs(5));
        send_response("HTTP/1.1 200 OK", "hello.html", &mut stream);
    } else {
        send_response("HTTP/1.1 404 Not Found", "404.html", &mut stream);
    }

    fn send_response(status_line : &str, file_path : &str, stream :&mut std::net::TcpStream) {
        let html = fs::read_to_string(file_path).unwrap();
        stream.write(format!("{}\r\n\r\n{}", status_line, html).as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
