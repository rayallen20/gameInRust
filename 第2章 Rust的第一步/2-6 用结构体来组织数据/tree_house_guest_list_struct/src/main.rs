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