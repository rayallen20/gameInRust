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