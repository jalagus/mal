use std::io::{stdin, stdout, Write};

use reader::MalType;

mod reader;
mod printer;

fn read(x: String) -> Result<MalType, String> {
    reader::read_str(x)
}

fn eval(x: MalType) -> MalType {
    x
}

fn print(x: MalType) -> String {
    printer::pr_str(&x)
}

fn rep(x: &String) -> String {
    match read(x.to_string()) {
        Ok(x) => print(eval(x)),
        Err(err_msg) => err_msg
    }
}

fn main() {
    let mut user_input = String::new();

    while user_input != "\n" {
        print!("user> ");
        let _ = stdout().flush();
        stdin().read_line(&mut user_input).expect("Error reading user input.");
        if user_input == "\n" {
            break;
        }
        
        if let Some('\n') = user_input.chars().next_back() {
            user_input.pop();
        }
        if let Some('\r') = user_input.chars().next_back() {
            user_input.pop();
        }

        println!("{}", rep(&user_input));
        user_input.clear();
    }
}