#[derive(Debug)]
struct Foo(i32);

impl Drop for Foo {
    fn drop(&mut self) {
        println!("drop {:?}", self);
    }
}

impl Foo {
    fn use_it(&self) {
        println!("consumed value {:?}", self);
    }
}

fn use_vale(v: &Foo) {
    println!("consumed value {:?}", v);
}

fn main() {
    let x = Foo(1);
    println!("before use x");
    // use_vale(&x);
    // use_vale(&x);
    (&x).use_it();
    (&x).use_it();
    println!("after use x");
}