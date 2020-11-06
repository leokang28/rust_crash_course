struct Fibs {
    x: u32,
    y: u32,
}

impl Fibs {
    fn new() -> Fibs {
        Fibs {
            x: 0,
            y: 1
        }
    }
}

impl Iterator for Fibs {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.y = self.x + self.y;
        self.x = self.y - self.x;

        Some(self.x)
    }
}

pub fn main() {
    for i in Fibs::new().take(10) {
        println!("{}", i);
    }
}