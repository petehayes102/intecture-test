#[macro_use]
extern crate inapi;

use inapi::{Error, Host, Payload};
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: incli run <server_host_or_ip>");
        exit(1);
    }

    if let Err(e) = run(&args[1]) {
        println!(""); // Output line break
        println!("{}", e);
        exit(1);
    }
}

fn run(name: &str) -> Result<(), Error> {
    print!("Connecting to host {}...", name);
    let mut host = try!(Host::connect(&format!("hosts/{}.json", name)));
    println!("done");

    // Call payloads
    let data = host.data_owned();
    for name in try!(needarray!(data => "/_payloads")) {
        println!("Running payload {}...", name);
        let payload = try!(Payload::new(try!(needstr!(name))));
        try!(payload.run(&mut host, None));
    }

    Ok(())
}
