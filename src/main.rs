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
    loop{
    println!("\nWhich encoding scheme would you like to use?
             \n 1) REP -> replicated encoding
             \n 2) PAR -> parity encoding 
             \n 3) EXIT
             \n");
    let mut scheme = String::new();
    io::stdin().read_line(&mut scheme).expect("Failed to read line");
    let scheme = scheme.trim();

    
        match scheme{
        "PAR" => parity_encoding(),
        "REP" => replicated_encoding(),
        "EXIT" => break,
        _ => println!("Not a valid encoding scheme"),
     }

    }
}

fn replicated_encoding(){

    println!("\nEnter your message\n");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    let message = message.trim().to_string();

    println!("\nInput the value of replicated bits \n");
    let mut rep = String::new();
    io::stdin().read_line(&mut rep).expect("Failed to read line");
    let rep: u32 = rep.trim().parse().ok().expect("parse panic");
    
    println!("\nInput the number of bits of error\n-> Remember the number of bits that our code can correct is {} \n",(rep-1)/2);
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t: u32 = t.trim().parse().ok().expect("parse panic");

    let replicated_code = encode_replicated(rep,&message);

    let transmitted_message = error_channel(t,&replicated_code);

    let res = detect_error_replicated(rep,&transmitted_message);
    let corrected_message = correct_replicated_error(rep, &transmitted_message);

    if res==false{
        println!("\nThe corrected message is {corrected_message} \n");
    }
} 

fn parity_encoding(){
    println!("\nInput your message \n");
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read line");
    message = message.trim().to_string();
    
    println!("\nInput the number of bits of error
            \n->Remember the number of error bits that can be detected has to be odd
            \n");
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t: u32 = t.trim().parse().ok().expect("parse panic");
    
    let parity_code = encode_parity(&message);

    let transmitted_message = error_channel(t,&parity_code);

    let res = detect_error_parity(&transmitted_message);

    println!("{res}");
}
