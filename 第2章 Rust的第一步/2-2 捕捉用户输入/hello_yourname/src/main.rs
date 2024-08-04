use std::io::stdin;

fn main() {
    println!("What's your name?");

    let mut your_name = String::new();
    // Tips: 变量是可变的 不代表变量的引用也是可变的
    stdin().read_line(&mut your_name).expect("Failed to read line");
    println!("Hello, {}!", your_name);
}
