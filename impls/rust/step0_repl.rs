use std::io::{stdin, stdout, Write};

fn read(x: String) -> String {
    x
}

fn eval(x: String) -> String {
    x
}

fn print(x: String) -> String {
    x
}

fn rep(x: &String) -> String {
    print(eval(read(x.to_string())))
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