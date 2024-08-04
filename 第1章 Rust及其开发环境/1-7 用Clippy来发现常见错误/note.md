# 1.7 用Clippy来发现常见错误

```rust
fn main() {
    let MYLIST = ["One", "Two", "Three"];
    for i in 0..3 {
        println!("{}", MYLIST[i]);
    }
}
```

```
cargo run
   Compiling clippy_example v0.1.0 (/clippy_example)
warning: variable `MYLIST` should have a snake case name
 --> src/main.rs:2:9
  |
2 |     let MYLIST = ["One", "Two", "Three"];
  |         ^^^^^^ help: convert the identifier to snake case: `mylist`
  |
  = note: `#[warn(non_snake_case)]` on by default

warning: `clippy_example` (bin "clippy_example") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.77s
     Running `target/debug/clippy_example`
One
Two
Three
```

使用`cargo clippy`命令查看建议列表:

```
cargo clippy
    Checking clippy_example v0.1.0 (/clippy_example)
warning: the loop variable `i` is only used to index `MYLIST`
 --> src/main.rs:3:14
  |
3 |     for i in 0..3 {
  |              ^^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
  = note: `#[warn(clippy::needless_range_loop)]` on by default
help: consider using an iterator
  |
3 |     for <item> in &MYLIST {
  |         ~~~~~~    ~~~~~~~

warning: variable `MYLIST` should have a snake case name
 --> src/main.rs:2:9
  |
2 |     let MYLIST = ["One", "Two", "Three"];
  |         ^^^^^^ help: convert the identifier to snake case: `mylist`
  |
  = note: `#[warn(non_snake_case)]` on by default

warning: `clippy_example` (bin "clippy_example") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.96s
```

1. 代码风格错误(`MYLIST`建议修改为`mylist`)
2. 遍历方式选择不当: 不建议使用索引来遍历列表.因为很有可能在遍历时改变了列表(比如删除了列表中的某个元素),但忘记更新索引的范围;建议使用迭代器

```rust
fn main() {
    let my_list = ["One", "Two", "Three"];
    for item in &my_list {
        println!("{}", item);
    }
}
```

```
cargo run
   Compiling clippy_fixed v0.1.0 (/clippy_fixed)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/clippy_fixed`
One
Two
Three
```

```
cargo clippy
    Checking clippy_fixed v0.1.0 (/clippy_fixed)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.46s
```