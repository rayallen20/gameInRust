# 2.3 将输入处理逻辑移入函数

```rust
use std::io::stdin;

fn main() {
    println!("What's your name?");
    let your_name = what_is_your_name();
    println!("Hello, {}!", your_name);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin().read_line(&mut your_name).expect("Failed to read line");
    your_name
}
```

```
cargo run
   Compiling hello_yourname v0.1.0 (/hello_yourname)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/hello_yourname`
What's your name?
fuck
Hello, fuck
!
```