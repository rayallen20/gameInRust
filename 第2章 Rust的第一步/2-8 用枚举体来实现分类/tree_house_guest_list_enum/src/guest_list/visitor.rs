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