use std::ops::Deref;

struct MyArray([u32; 5]);

impl MyArray {
    fn new() -> MyArray {
        MyArray([1;5])
    }
}

impl Deref for MyArray {
    type Target = [u32];

    fn deref(& self) -> & Self::Target {
        & self.0
    }
}

pub fn main() {
    let my_array = MyArray::new();
    let _: &MyArray = & my_array;
    let _: &[u32] = & my_array;
}