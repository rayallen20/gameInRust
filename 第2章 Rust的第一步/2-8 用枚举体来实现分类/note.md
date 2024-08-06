# PART1. 需求

- 存储一个与访客相关联的行为,该行为包括:
  - 允许进入
  - 允许进入并说出定制欢迎语
  - 禁止进入
  - 临时体验会员
- 存储访客年龄,并在访客年龄小于21岁时,禁止饮酒
- 对新访客设置临时体验会员

# PART2. 实现

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    ├── guest_list
    │         ├── action.rs
    │         ├── mod.rs
    │         ├── visitor.rs
    │         └── visitor_list.rs
    ├── lib.rs
    └── main.rs

2 directories, 8 files
```

## 2.1 存储行为并对新访客设置临时体验会员

`src/guest_list/action.rs`

```rust
#[derive(Debug)]
pub enum Action {
    Accept,
    AcceptWithNote {note: String},
    Refuse,
    Probation,
}
```

`src/guest_list/mod.rs`

```rust
mod visitor;
use visitor::Visitor;

mod visitor_list;
pub use visitor_list::VisitorList;
mod action;
pub use action::Action;
```

`src/guest_list/visitor.rs`

```rust
use crate::guest_list::Action;

#[derive(Debug)]
pub(crate) struct  Visitor {
    pub(crate) name: String,
    pub(crate) action: Action,
}

impl Visitor {
    pub(crate) fn new(name: &str, action: Action) -> Visitor {
        Visitor {
            name: name.to_lowercase(),
            action,
        }
    }

    pub(crate) fn greet(&self) {
        match &self.action {
            Action::Accept => println!("Welcome come to the tree house {}!", self.name),
            Action::AcceptWithNote {note} => println!("Welcome come to the tree house {} (note: {})", self.name, note),
            Action::Refuse => println!("Do not allow{} in", self.name),
            Action::Probation => println!("{} is now a probationary member", self.name)
        }
    }
}
```

- 打招呼用语不再受外部控制,而是受`Action`的控制
- 因此关联函数`Visitor::new()`和成员方法的形参有所改变,不再需要`greeting`参数,而是`action`参数

`src/guest_list/visitor_list.rs`

```rust
use crate::guest_list::{Action, Visitor};

#[derive(Debug)]
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

    pub fn add(&mut self, name: &str, action: Action) {
        let visitor = Visitor::new(name, action);
        match self.find(name) {
            Some(_) => println!("{} is already on the guest list.", name),
            None => self.visitor_list.push(visitor)
        }
    }

    fn find(&self, name: &str) -> Option<&Visitor> {
        self.visitor_list.iter().find(|visitor| visitor.name.to_lowercase() == name)
    }

    pub fn greet(&mut self, name: &str) {
        match self.find(name) {
            Some(visitor) => visitor.greet(),
            None => {
                self.add(name, Action::Probation);
                // 添加后再次查找访客
                if let Some(visitor) = self.find(name) {
                    visitor.greet();
                }
            }
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

`src/main.rs`:

```rust
use std::io::stdin;
use tree_house_guest_list_enum::guest_list::{Action, VisitorList};

fn main() {
    let mut visitor_list = make_visitor_list();

    loop {
        println!("What's your name? (Leave empty and press ENTER to quit)");
        let your_name = what_is_your_name();
        if your_name.is_empty() {
            break;
        }
        visitor_list.greet(&your_name);
    }

    println!("The final guest list:");
    println!("{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin().read_line(&mut your_name).expect("Failed to read line");
    // 防止访客输入大小写字母混合的名字 故全部转换为小写
    your_name.trim().to_lowercase().to_string()
}

fn make_visitor_list() -> VisitorList {
    let mut visitor_list = VisitorList::new();
    visitor_list.add("fuck",  Action::Accept);
    visitor_list.add("shit",  Action::AcceptWithNote {note: String::from("脱脂牛奶在冰箱里")});
    visitor_list.add("mother_fucker", Action::Refuse);
    visitor_list
}
```

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/tree_house_guest_list_enum`
What's your name? (Leave empty and press ENTER to quit)
fuck
Welcome come to the tree house fuck!
What's your name? (Leave empty and press ENTER to quit)
shit
Welcome come to the tree house shit (note: 脱脂牛奶在冰箱里)
What's your name? (Leave empty and press ENTER to quit)
mother_fucker
Do not allowmother_fucker in
What's your name? (Leave empty and press ENTER to quit)
apple
apple is now a probationary member
What's your name? (Leave empty and press ENTER to quit)

The final guest list:
VisitorList {
    visitor_list: [
        Visitor {
            name: "fuck",
            action: Accept,
        },
        Visitor {
            name: "shit",
            action: AcceptWithNote {
                note: "脱脂牛奶在冰箱里",
            },
        },
        Visitor {
            name: "mother_fucker",
            action: Refuse,
        },
        Visitor {
            name: "apple",
            action: Probation,
        },
    ],
}
```

## 2.2 在访客年龄小于21岁时,禁止饮酒

`src/guest_list/visitor.rs`:

```rust
use crate::guest_list::Action;

#[derive(Debug)]
pub(crate) struct  Visitor {
    pub(crate) name: String,
    pub(crate) action: Action,
    pub(crate) age: u8,
}

impl Visitor {
    pub(crate) fn new(name: &str, action: Action, age: u8) -> Visitor {
        Visitor {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    pub(crate) fn greet(&self) {
        // 这里不能写self.action 因为Action::AcceptWithNote.note在这个场景中
        // 仅仅需要被读取 而不需要获取其所有权
        // 如果写成self.action 则意味着要在这个match表达式中获取其所有权
        match &self.action {
            Action::Accept => println!("Welcome come to the tree house {}!", self.name),
            Action::AcceptWithNote {note} => {
                println!("Welcome come to the tree house {} (note: {})", self.name, note);
                if self.age < 21 {
                    println!("{}, 请勿饮酒", self.name);
                }
            },
            Action::Refuse => println!("Do not allow{} in", self.name),
            Action::Probation => println!("{} is now a probationary member", self.name)
        }
    }
}
```

`src/guest_list/visitor_list.rs`:

```rust
use crate::guest_list::{Action, Visitor};

#[derive(Debug)]
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

    pub fn add(&mut self, name: &str, action: Action, age: u8) {
        let visitor = Visitor::new(name, action, age);
        match self.find(name) {
            Some(_) => println!("{} is already on the guest list.", name),
            None => self.visitor_list.push(visitor)
        }
    }

    fn find(&self, name: &str) -> Option<&Visitor> {
        self.visitor_list.iter().find(|visitor| visitor.name.to_lowercase() == name)
    }

    pub fn greet(&mut self, name: &str) {
        match self.find(name) {
            Some(visitor) => visitor.greet(),
            None => {
                // 临时访客不需要确认其年龄
                self.add(name, Action::Probation, 0);
                // 添加后再次查找访客
                if let Some(visitor) = self.find(name) {
                    visitor.greet();
                }
            }
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

`src/main.rs`:

```rust
use std::io::stdin;
use tree_house_guest_list_enum::guest_list::{Action, VisitorList};

fn main() {
    let mut visitor_list = make_visitor_list();

    loop {
        println!("What's your name? (Leave empty and press ENTER to quit)");
        let your_name = what_is_your_name();
        if your_name.is_empty() {
            break;
        }
        visitor_list.greet(&your_name);
    }

    println!("The final guest list:");
    println!("{:#?}", visitor_list);
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin().read_line(&mut your_name).expect("Failed to read line");
    // 防止访客输入大小写字母混合的名字 故全部转换为小写
    your_name.trim().to_lowercase().to_string()
}

fn make_visitor_list() -> VisitorList {
    let mut visitor_list = VisitorList::new();
    visitor_list.add("fuck",  Action::Accept, 45);
    visitor_list.add("shit",  Action::AcceptWithNote {note: String::from("脱脂牛奶在冰箱里")}, 15);
    visitor_list.add("mother_fucker", Action::Refuse, 30);
    visitor_list
}
```

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/tree_house_guest_list_enum`
What's your name? (Leave empty and press ENTER to quit)
fuck
Welcome come to the tree house fuck!
What's your name? (Leave empty and press ENTER to quit)
shit
Welcome come to the tree house shit (note: 脱脂牛奶在冰箱里)
shit, 请勿饮酒
What's your name? (Leave empty and press ENTER to quit)
mother_fucker
Do not allowmother_fucker in
What's your name? (Leave empty and press ENTER to quit)
apple
apple is now a probationary member
What's your name? (Leave empty and press ENTER to quit)

The final guest list:
VisitorList {
    visitor_list: [
        Visitor {
            name: "fuck",
            action: Accept,
            age: 45,
        },
        Visitor {
            name: "shit",
            action: AcceptWithNote {
                note: "脱脂牛奶在冰箱里",
            },
            age: 15,
        },
        Visitor {
            name: "mother_fucker",
            action: Refuse,
            age: 30,
        },
        Visitor {
            name: "apple",
            action: Probation,
            age: 0,
        },
    ],
}
```