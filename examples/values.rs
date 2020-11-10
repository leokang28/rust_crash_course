fn main() {
    let number = 1;
    let strs = String::from("哈哈");

    let strs_ref = String::from("嘻嘻");

    print_num(number);
    print_num(number);

    // solution 1 clone
    print_str(strs.clone());
    print_str(strs);

    // solution 2 ref

    print_str_ref(&strs_ref);
    print_str_ref(&strs_ref);
}

fn print_num(n: i32) {
    println!("{}", n);
}

fn print_str(s: String) {
    println!("{}", s);
}

fn print_str_ref(s: &String) {
    println!("{}", s);
}
