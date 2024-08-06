use std::io::stdin;

fn main() {
    println!("What's your name?");
    let your_name = what_is_your_name();
    println!("Hello, {}!", your_name);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin().read_line(&mut your_name).expect("Failed to read line");
    your_name.trim().to_string()
}