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