# 2.2 捕捉用户输入

## 2.2.1 提示访客输入姓名

```rust
fn main() {
    println!("What's your name?");
}
```

## 2.2.2 存储用户的输入

```rust
fn main() {
    println!("What's your name?");
    
    let mut your_name = String::new();
}
```

## 2.2.3 接收键盘输入 && 2.2.4 读取用户输入

```rust
use std::io::stdin;

fn main() {
    println!("What's your name?");

    let mut your_name = String::new();
    // Tips: 变量是可变的 不代表变量的引用也是可变的
    stdin().read_line(&mut your_name).expect("Failed to read line");
}
```

1. 和其他编程语言不同,Rust的数据所有权概念使得此处的`read_line()`方法只能接收可变引用,而不能直接接收`your_name`这个可变变量
    - 因为如果直接接收`your_name`这个可变变量,那么`read_line()`方法就会拥有`your_name`的所有权,而`main()`函数中后续就不能再使用`your_name`了
2. `Result.expect()`方法是一个简单的错误处理方法,如果`read_line()`方法返回`Err`,那么`expect()`方法会抛出一个错误信息
    - 另一个常用的方法是`Result.unwrap()`,如果`read_line()`方法返回`Err`,那么`unwrap()`方法会直接终止程序并抛出panic,panic中包含自定义的错误信息
3. 可变引用意味着可以通过引用修改变量的值

## 2.2.5 用占位符实现格式化打印

```rust
use std::io::stdin;

fn main() {
    println!("What's your name?");

    let mut your_name = String::new();
    // Tips: 变量是可变的 不代表变量的引用也是可变的
    stdin().read_line(&mut your_name).expect("Failed to read line");
    println!("Hello, {}!", your_name);
}
```

运行结果:

```
cargo run
   Compiling hello_yourname v0.1.0 (/hello_yourname)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.43s
     Running `target/debug/hello_yourname`
What's your name?
fuck
Hello, fuck
!
```

是的,你看到`!`被换行打印了,这是因为用户的输入本身带有一个换行,后续我们将修复这个问题