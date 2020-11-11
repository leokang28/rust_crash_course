struct Single<T> {
    next: Option<T>
}

impl<T> Iterator for Single<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        // solution 1
        // let mut res = None;
        // std::mem::swap(&mut res, &mut self.next);
        // res

        // solution 2
        // std::mem::replace(&mut self.next, None)

        // solution 3
        self.next.take()
    }
}

fn single_fact<T> (t: T) -> Single<T> {
    Single {
        next: Some(t),
    }
}

pub fn main() {
    let collect:Vec<i32> = single_fact(42).collect();
    // println!("{:?}", collect);

    assert_eq!(vec![42], collect);

}