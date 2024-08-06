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