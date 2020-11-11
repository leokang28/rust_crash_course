fn msg_and_return<'a, 'b>(msg: &'a String, ret: &'b String) -> &'b String {
    println!("{}", msg);
    ret
}

fn foo(name: &String) -> &String {
    let msg = "this is the message".to_string();
    msg_and_return(&msg, name)

}

pub fn main() {
    let name = "leo".to_string();
    let ret = foo(&name);
    println!("{}", ret);
}