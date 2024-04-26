/**
 * @author Leviathen
**/

use std::io::{stdin, stdout, Write};

#[derive(Debug)]
struct User{
    name: String
}




mod commands {
    use std::io::{stdin, stdout, Write};

    use crate::User;


    pub fn users(args: Vec<&str>, users: &mut Vec<User>){
        if args.len() == 1 {
            let mut index: u32 = 0;
            for user in users {
                println!("{} - {}",index,user.name);
                index += 1;
            }
        }else{
            if args.get(1).unwrap().to_string() == String::from("new") {
                print!("name> ");
                stdout().flush().expect("Unable to flush stdout.");
            
                let mut name = String::new();
                stdin().read_line(&mut name).unwrap();
                users.push(User { name: name });
                        
            }
        }
    }
}

fn main(){
    let mut users: Vec<User> = vec![];
    loop {
    
    
        print!("> ");
        stdout().flush().expect("Unable to flush stdout.");
    
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    
        let parts: Vec<_> = input.trim().split_whitespace().collect();
        let command:&str = parts.get(0).unwrap();
        let args: Vec<&str> = parts;
        
        match command {
            "users"=>{commands::users(args,&mut users)}
            _=>{println!("\"{}\" Is not a valid command.",command)}
        }
    }
} 