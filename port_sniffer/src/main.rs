use std::env;
use std::net::{IpAddr, SocketAddr};
use std::process;
use std::sync::mpsc::{Sender, self};
use std::thread;
use std::net::TcpStream;

const MAX: u16 = 65535;

#[derive(Debug, Copy, Clone)]
struct Arguments {
    ip : IpAddr,
    start_port: u16,
    end_port: u16,
    thread_num : u16
}

impl Arguments {
    fn new(args : &Vec<String>) -> Result<Arguments, &'static str> {
        let size = args.len();
        if size < 2 {
            print_usage();
            process::exit(0);
        }
        // if contains -h just return
        if args.contains(&String::from("-h")) {
            print_usage();
            process::exit(0);
        }
        let mut thread_num:u16 = 4;
        let mut index = 1;
        if args[index].eq(&String::from("-t")) {
            index += 1;
            thread_num = match args[index].parse() {
                Ok(s) => s,
                Err(_) => return Err("-t param must be number"),
            };
            index +=1;
        }
        if index >= size {
            return Err("ipAddr must be present");
        }
        let ip : IpAddr = match args[index].parse() {
            Ok(s) => s,
            Err(_) => return Err("ip format is not right"),
        };
        index += 1;
        let start_port:u16 = if index < size {
            match args[index].parse() {
                Ok(s) => s,
                Err(_) => return Err("startPort format is not right")
            }
        } else {
            1
        };
        index += 1;
        let end_port:u16 = if index < size {
            match args[index].parse() {
                Ok(s) => s,
                Err(_) => return Err("endPort format is not right")
            }
        } else {
            MAX
        };
        if end_port < start_port {
            return Err("end should be greater than start");
        }
        Ok(Arguments{ip, start_port, end_port, thread_num})
    }
}

fn print_usage() {
    println!("
Usage: port_sniffer [Option]... ip [startPort] [endPort]
    -h help
    -t thread num     
    ");
}

///
/// scan ip [begin, end]
/// 
fn scan(ip:IpAddr, start:u16, end:u16, thread_num:u16, sender:Sender<u16>) {
    let mut start_mut = start;
    loop {
        if start_mut > end {
            return;
        }
        if let Ok(_) = TcpStream::connect(&SocketAddr::new(ip, start_mut)) {
            sender.send(start_mut).unwrap();
        }
        start_mut += thread_num;
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();
    let arg = Arguments::new(&args).unwrap_or_else( |e| {   
        eprint!("{}", e);
        process::exit(0);
    });
    println!("{:?}", arg);
    let num = arg.thread_num;
    let (tx, rx) = mpsc::channel::<u16>();
    for i in 0..num {
        let sender = tx.clone();
        thread::spawn(move || {
            scan(arg.ip, arg.start_port + i, arg.end_port, num, sender);
        });
    }
    // drop the tx because no move to it
    drop(tx);
    for r in rx {
        println!("{} port open", r);
    }
    println!("finished");
}
