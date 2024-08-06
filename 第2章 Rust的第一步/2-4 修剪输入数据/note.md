# 2.4 修剪输入数据

我们先使用`{:?}`占位符来打印数据,这个占位符被称为调试占位符,它比`{}`更加详细

```rust
use std::io::stdin;

fn main() {
    println!("What's your name?");
    let your_name = what_is_your_name();
    println!("Hello, {:?}!", your_name);
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
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running `target/debug/hello_yourname`
What's your name?
fuck
Hello, "fuck\n"!
```

这个`\n`是用户输入完毕后敲回车时带过来的输入,其实并不属于我们应该打印的内容.

解决方案:使用`String.trim()`方法,去除字符串两端的空白字符(这里的空白字符包括空格、制表符(`\t`)、换行符(`\n`))

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
    your_name.trim().to_string()
}
```

```
cargo run
   Compiling hello_yourname v0.1.0 (/hello_yourname)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/hello_yourname`
What's your name?
fuck
Hello, fuck!
```