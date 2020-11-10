struct InfiniteUnit;

// solution 1
// impl IntoIterator for InfiniteUnit {
//     type Item = ();
//     type IntoIter = InfiniteUnitIter;
//     fn into_iter(self) -> Self::IntoIter {
//         InfiniteUnitIter
//     }
// }

struct InfiniteUnitIter;

impl Iterator for InfiniteUnitIter {
    type Item = ();
    fn next(&mut self) -> Option<()> {
        Some(())
    }
}

// solution 2
impl IntoIterator for InfiniteUnit {
    type Item = ();
    type IntoIter = std::iter::Repeat<()>;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::repeat(())
    }
}

pub fn main() {
    let mut count = 0;
    for _ in InfiniteUnit {
        if count > 10 {
            break;
        }
        count += 1;
        println!("count = {}", count);
    }
}
