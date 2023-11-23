extern crate rand;

//use std::error;

use rand::Rng;

pub fn error_channel(mut t : u32, message : &String) -> String{

    println!("Message before transmission is {}",message);

    let mut new_message = message.clone();
    let mut error_count = 0;
    let mut i = 0;

    for ch in message.chars(){

        let error_bit = rand::thread_rng().gen_range(0..=1);

        if ch != char::from_digit(error_bit as u32, 10).unwrap() && t > 0{
            new_message.replace_range(i..i+1,&error_bit.to_string());
            error_count+=1;
            t-=1;
        }
        i+=1;
        
    }

    println!("Number of error bits introduced are {error_count}");
    println!("The message after transmission is {}",new_message);
    new_message
}   
