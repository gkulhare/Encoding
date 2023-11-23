pub fn encode_replicated(rep : u32, message : &String) -> String{

    let mut new_message = String::new();

    let message = message.trim();

    for ch in message.chars(){
        for _i_ in 0..rep{
            new_message.push(ch);
        }
    }

    println!("Here is your message: {}",new_message.trim());
   new_message
}

pub fn detect_error_replicated(rep : u32, message : &String) -> bool{
    
    let mut count =0;
    let mut bit = '0';
    
    // for i in (0..message.len()).step_by(rep as usize){

    //     for 
    //     println!("{}",message.chars().nth(i).unwrap())
    // }

    for i in 0..message.len(){
        if i%rep as usize==0 {
            bit = message.chars().nth(i).unwrap();
        }
        else{
            if bit!=message.chars().nth(i).unwrap(){
                
                println!("There was an error");
                return false;
            }
            else{
                continue;
            }
        } 
        println!("bit is {} ch is {} at index {}",bit,message.chars().nth(i).unwrap() , count);
        count+=1;
    }
    println!("There were no errors");
    true
}