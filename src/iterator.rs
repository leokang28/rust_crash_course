struct Empty;

impl Iterator for Empty {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        Some(10)
    }
}

pub fn run() {
    for i in Empty.take(2) {
        println!("i = {}", i);
    }
    println!("main end");
}

pub fn main() {
    run();
}
