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