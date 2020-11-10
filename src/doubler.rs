struct Doubler<I> {
    iter: I,
}

impl<I> Iterator for Doubler<I>
where
    I: Iterator,
    I::Item: std::ops::Add<Output = I::Item> + Copy,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            None => None,
            Some(v) => Some(v + v),
        }
    }
}

fn run() {
    let iter = Doubler { iter: 1..11 };
    for i in iter {
        println!("{}", i);
    }
}

pub fn main() {}
