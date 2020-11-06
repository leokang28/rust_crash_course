use std::fmt::{
    Display,
    Error,
    Formatter,
};

mod iterator;

mod fibs;

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
    // iterator::main();
    // fibs::main();
    let a: u8 = 254;
    match a.checked_add(2) {
        None => println!("overflow, max of u8 is {}", u8::MAX),
        Some(v) => println!("{}", v),
    };
}


