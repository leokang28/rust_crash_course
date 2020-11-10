#[derive(Debug)]
struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop {:?}", self);
    }
}

fn main() {
    println!("before _x");
    let _x = Foo(1);
    println!("after _x");
    {
        println!("before _y");
        let _y = Foo(2);
        println!("after _y");
    }
    println!("end main");
}
