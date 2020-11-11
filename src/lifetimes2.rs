struct Person {
    name: String,
    age: u8,
}

fn get_name(p: &Person) -> &String {
    & p.name
}

fn get_older_name<'a>(p1: &'a Person, p2: &'a Person) -> &'a String {
    // 至少有相同的生命周期，如果某个生命周期稍长也无妨
    if p1.age > p2.age {
        & p1.name
    } else {
        & p2.name
    }
}

pub fn main() {
    let leo = Person {
        name: "Leo".to_string(),
        age: 25
    };

    foo(&leo);

}

fn foo(leo: &Person) {
    let jl = Person {
        name: "JL".to_string(),
        age: 26,
    };

    let name = get_older_name(leo, &jl);
    println!("name is {}", name);
}