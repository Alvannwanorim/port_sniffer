use std::env;
use std::io::{self, Write};

use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

const MAX: u16 = 16;
struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    thread: u16
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, & 'static str> {
        if args.len() < 2 {
            return  Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too mony arguments")
        }

        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f){
            return  Ok(Arguments{ flag: String::from(""), ipaddr, thread: 4});
        }else {
            
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -j to select how many threads you want 
                \r\n       -h or -help to show this help message");
                return  Err("help");
            }else if flag.contains("-h") || flag.contains("--help") {
                return  Err("too many arguments");
            }else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("not a valid APADDR; must be of type IPV4 or IPV6")
                };

                let thread = match  args[2].parse::<u16>() {
                    Ok(s)  => s,
                    Err(_) => return  Err("failed to parse thread number")
                };

                return Ok(Arguments{thread, flag, ipaddr})
            }else {
                return Err("invalid syntax");
            }
        }
    }
}
fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16){
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    println!("{:?}",&args);
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        }else {
             eprintln!("{} problem parsing arguments: {}", program,err);
             process::exit(0)
        }
    }
    );

    let num_threads = arguments.thread;
    let addr = arguments.ipaddr;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads)
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!(".");
    out.sort();
    for v in out {
        print!("{} is open", v)
    }
}

