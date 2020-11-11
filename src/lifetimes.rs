#[derive(Debug)]
struct Person {
    name: Option<String>,
    age: Option<u8>,
}

fn print_person(mut person: Person) {
    match person.name {
        None => println!("no name provided"),
        Some(ref name) => println!("name is {}", name),
    }

    match person.age {
        None => println!("no age provided "),
        Some(ref mut age) => {
            println!("age is {}", age);
            *age += 1;
        },
    }

    println!("{:?}", person);

}

pub fn main() {

    print_person(Person {
        name: Some("Leo".to_string()),
        age: Some(20),
    })

}