# 2.7 用向量来存储可变的数据

## PART1. 需求

1. 若用户输入的姓名为空,则结束程序
    - 现在是用户输入后直接结束程序的
2. 若用户输入的姓名是一个新访客,则将其添加到访客列表中
3. 在程序退出前,打印访客列表

## PART2. 实现

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
#[derive(Debug)]
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

- 添加了`#[derive(Debug)]`属性,可以直接使用`{:?}`打印`Visitor`对象

`src/guest_list/visitor_list.rs`:

```rust
use crate::guest_list::Visitor;

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

    pub fn greet(&mut self, name: &str) {
        match self.find(name) {
            Some(visitor) => visitor.greet(),
            None => {
                let greeting = format!("Hello {}, new friend!", name);
                self.add(name, greeting.as_str());
                println!("{}", greeting);
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

- 添加了`#[derive(Debug)]`属性,可以直接使用`{:?}`打印`VisitorList`对象
- `greet()`方法中,若找不到对应的访客,则会添加一个新的访客
  - 该方法的参数由`&self`修改为了`&mut self`,因为在添加新访客时,需要调用`add()`方法 
  - 这里`greeting`变量的所有权并没有转移给`add()`方法,因为调用`add()`方法时,使用了`as_str()`方法,将一个`&str`类型的引用传递给了`add()`方法

`src/main.rs`:

```rust
use std::io::stdin;
use tree_house_guest_list_vector::guest_list::VisitorList;

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
    visitor_list.add("fuck", "fuck u");
    visitor_list.add("shit", "eat shit");
    visitor_list.add("mother_fucker", "Shut up, u mother fucker bitch");
    visitor_list
}
```

- 使用`loop`循环来检测退出
- 由于`visitor_list.greet()`方法要求一个`&mut self`,因此声明`VisitorList`对象时,要声明为可变变量

```
cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/tree_house_guest_list_vector`
What's your name? (Leave empty and press ENTER to quit)
fuck
fuck u
What's your name? (Leave empty and press ENTER to quit)
shit
eat shit
What's your name? (Leave empty and press ENTER to quit)
apple
Hello apple, new friend!
What's your name? (Leave empty and press ENTER to quit)

The final guest list:
VisitorList {
    visitor_list: [
        Visitor {
            name: "fuck",
            greeting: "fuck u",
        },
        Visitor {
            name: "shit",
            greeting: "eat shit",
        },
        Visitor {
            name: "mother_fucker",
            greeting: "Shut up, u mother fucker bitch",
        },
        Visitor {
            name: "apple",
            greeting: "Hello apple, new friend!",
        },
    ],
}
```