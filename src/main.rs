use std::fmt::{
    Display,
    Error,
    Formatter,
};


struct Person {
    name: String,
    age: u8,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}, {}", self.name, self.age)
    }
}

fn main() {
    let leo = Person {
        name: "leo".to_string(),
        age: 25
    };
    println!("Hello {}", leo);
}
