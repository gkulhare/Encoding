pub fn encode_parity(message : &String) -> String{

    let mut new_message = message.trim().to_string();
    // println!("Here is your number {message}");

    let mut count: u32=0;

    for ch in message.chars(){
        if ch=='1'{
            count+=1;
        }
    }

    if count%2==0{
       new_message.push('0');

    }

    else{
        new_message.push('1');
    }

   new_message
}

pub fn detect_error_parity(message : &String) -> String{

    let len = message.len();
    let recreated_message = encode_parity(&message[0..len-1].to_string());

    //println!("last recreated bit is {} last original bit is {}",&recreated_message[len-1..len],&message[len-1..len]);    
    
    if recreated_message[len-1..len] == message[len-1..len]{
        return ("\nThe message is safe\n").to_string();
    }
    else{
        return ("\nThe message has been compromised\n").to_string();
    }
}