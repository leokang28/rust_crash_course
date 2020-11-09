fn call_a_func<F> (f: F)
    where F: Fn() -> ()
{
    f();
}

pub fn main() {
    let something = "something";
    let say_something = || println!("{}", something);
    call_a_func(say_something);
}