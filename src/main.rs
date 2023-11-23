// extern crate rand;

pub mod error_channel;
pub mod replicated_encoding;
pub mod parity_encoding;

use std::io;

use replicated_encoding::*;
use error_channel::error_channel;
use parity_encoding::*;

fn main() {

    //Encoding 
    println!("Input your message");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    message = message.trim().to_string();

    println!("Input the value of t");
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t: u32 = t.trim().parse().ok().expect("parse panic");

    println!("Input the value of replicated bits");
    let mut rep = String::new();
    io::stdin().read_line(&mut rep).expect("Failed to read line");
    let rep: u32 = rep.trim().parse().ok().expect("parse panic");

    // let parity_code = encode_parity(&message);

    let replicated_code = encode_replicated(rep,&message);
    // println!("Encoded message is {res}");

    let transmitted_message = error_channel(t,&replicated_code);

    let res = detect_error_replicated(rep,&transmitted_message);
}

