/**
 * @author Leviathen
**/

use std::io::{stdin, stdout, Write};

#[derive(Debug)]
#[derive(Clone)]
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
                print!("{} - {}",index,user.name);
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
    pub fn select(args: Vec<&str>, user: &mut User, users: &mut Vec<User>){
        if args.len() == 1 {
            print!("Invalid use of \"select\". Select takes 2 arguments");
        }else{
            let selected: &str = args.get(1).unwrap();
            match selected{
                "user"=>{
                    let s_user: usize = args.get(2).unwrap().parse().expect(format!("Expected int, got: \"{}\".",args.get(2).unwrap()).as_str());
                    if let Some(selected_user) = users.get(s_user) {
                        *user = selected_user.clone();
                        print!("Selected User: {}",user.name);
                    } else {
                        print!("User index out of bounds.");
                    }
                }
                _ =>{}
            }
        }

    }
    pub fn selected(args: Vec<&str>, users: &mut Vec<User>,scat: &mut String){
        
    }

}

fn main(){
    let mut users: Vec<User> = vec![];
    let mut selected_user: User = User { name: String::from("") };
    let mut scat: String = String::new();
    loop {
    
    
        print!("> ");
        stdout().flush().expect("Unable to flush stdout.");
    
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    
        let parts: Vec<_> = input.trim().split_whitespace().collect();
        let command:&str = parts.get(0).unwrap_or(&&"errornocmd");
        let args: Vec<&str> = parts;
        
        match command {
            "users"=>{commands::users(args,&mut users)},
            "select"=>{commands::select(args, &mut selected_user, &mut users)}
            "selected"=>{commands::selected(args,&mut users,&mut scat)}
            "errornocmd"=>{}
            _=>{println!("\"{}\" Is not a valid command.",command)}
            
        }
    }
} 