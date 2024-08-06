# 2.6 用结构体来组织数据

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── guest_list
    │         ├── mod.rs
    │         ├── visitor.rs
    │         └── visitor_list.rs
    ├── lib.rs
    └── main.rs

2 directories, 7 files
```

`src/guest_list/visitor.rs`:

```rust
pub(crate) struct  Visitor {
    pub(crate) name: String,
    pub(crate) greeting: String
}

impl Visitor {
    pub(crate) fn new(name: &str, greeting: &str) -> Visitor {
        Visitor {
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }

    pub(crate) fn greet(&self) {
        println!("{}", self.greeting);
    }
}
```

`src/guest_list/visitor_list.rs`:

```rust
use crate::guest_list::Visitor;

pub struct  VisitorList {
    visitor_list: Vec<Visitor>
}

impl VisitorList {
    pub fn new() -> VisitorList {
        VisitorList {
            // 预先分配10个空间 以减少内存分配次数
            visitor_list: Vec::with_capacity(10)
        }
    }

    pub fn add(&mut self, name: &str, greeting: &str) {
        let visitor = Visitor::new(name, greeting);
        match self.find(name) {
            Some(_) => println!("{} is already on the guest list.", name),
            None => self.visitor_list.push(visitor)
        }
    }

    fn find(&self, name: &str) -> Option<&Visitor> {
        self.visitor_list.iter().find(|visitor| visitor.name.to_lowercase() == name)
    }

    pub fn greet(&self, name: &str) {
        match self.find(name) {
            Some(visitor) => visitor.greet(),
            None => println!("Sorry, {} is not on the guest list.", name)
        }
    }
}

// 等价于为VisitorList添加了#[derive(Default)]属性
impl Default for VisitorList {
    fn default() -> Self {
        Self::new()
    }
}
```

注: 这里实现Default Trait是`cargo clippy`命令检查出来的

`src/guest_list/mod.rs`:

```rust
mod visitor;
use visitor::Visitor;

mod visitor_list;
pub use visitor_list::VisitorList;
```

这个模块整体设计的意图:

1. 不让外部代码直接访问`Visitor`实例
2. 通过`VisitorList`来管理`Visitor`实例
3. 外部可以访问`VisitorList`实例

`src/main.rs`:

```rust
use std::io::stdin;
use tree_house_guest_list_struct::guest_list::VisitorList;

fn main() {
    let visitor_list = make_visitor_list();

    println!("What's your name?");
    let your_name = what_is_your_name();
    visitor_list.greet(&your_name);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin().read_line(&mut your_name).expect("Failed to read line");
    // 防止访客输入大小写字母混合的名字 故全部转换为小写
    your_name.trim().to_lowercase().to_string()
}

fn make_visitor_list() -> VisitorList {
    let mut visitor_list = VisitorList::new();
    visitor_list.add("fuck", "fuck u");
    visitor_list.add("shit", "eat shit");
    visitor_list.add("mother_fucker", "Shut up, u mother fucker bitch");
    visitor_list
}
```

```
cargo run
   Compiling tree_house_guest_list_struct v0.1.0 (/tree_house_guest_list_struct)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 6.04s
     Running `target/debug/tree_house_guest_list_struct`
What's your name?
fuck
fuck u
```

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/tree_house_guest_list_struct`
What's your name?
abc
Sorry, abc is not on the guest list.
```