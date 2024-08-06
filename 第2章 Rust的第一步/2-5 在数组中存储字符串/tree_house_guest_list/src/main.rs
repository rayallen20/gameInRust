use std::io::stdin;

fn main() {
    println!("What's your name?");
    let your_name = what_is_your_name();
    // 此处发生了解引用强制转换 String实现了Deref trait
    // 因此自动调用了Deref::deref方法 得到了一个&str
    if is_visitor(&your_name) {
        println!("Welcome to the tree house, {}!", your_name);
    } else {
        println!("Sorry, you are not on the guest list.");
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin().read_line(&mut your_name).expect("Failed to read line");
    // 防止访客输入大小写字母混合的名字 故全部转换为小写
    your_name.trim().to_lowercase().to_string()
}

fn is_visitor(name: &str) -> bool {
    let visitors = ["fuck", "shit", "fuck_shit"];
    let mut is_visitor = false;
    for visitor in visitors {
        if name == visitor {
            is_visitor = true;
            break;
        }
    }
    is_visitor
}