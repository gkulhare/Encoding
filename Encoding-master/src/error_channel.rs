extern crate rand;

//use std::error;

use rand::Rng;
use fancy::printcol;

pub fn error_channel(mut t : u32, message : &String) -> String{

    println!("\nThe message before transmission is {}\n",message);

    let mut new_message = message.clone();
    //let mut error_count = 0;
    // let mut i = 0;

    // for ch in message.chars(){

    //     let error_bit = rand::thread_rng().gen_range(0..=1);

    //     if ch != char::from_digit(error_bit as u32, 10).unwrap() && t > 0{
    //         new_message.replace_range(i..i+1,&error_bit.to_string());
    //         error_count+=1;
    //         t-=1;
    //     }
    //     i+=1;
        
    // }

    while t>0{
        let error_index =  rand::thread_rng().gen_range(0..1000)%new_message.len();

        println!("\nChanging the bit at {error_index}\n");

        let mut ch = new_message.chars().nth(error_index).unwrap();

        if ch=='0'{
            ch='1';
        }
        else{
            ch='0';
        }

        new_message.replace_range(error_index..error_index+1,&ch.to_string());
        // error_highlight.replace_range(error_index..error_index+1, )

        t-=1;

    }

    //println!("Number of error bits introduced are {error_count}");
    print!("\nThe message after transmission is ");
    for i in 0..new_message.len(){

        let ch = new_message.chars().nth(i).unwrap();
        if ch!=message.chars().nth(i).unwrap(){
            printcol!("[bold|red]{}",ch);
        }
        else{
            print!("{ch}");
        }
    }
    print!("\n");
    
    new_message
}   
