

pub fn main() {
    let b1: &[u8; 22] = b"hello world in binary!";

    let b2: &[u8] = b"hello world in binary!";

    println!("{:?}", b1);
    println!("{:?}", b2);

    for arg in std::env::args() {
        let chars = arg.chars();
        println!(
            "arg: {}, chars: {}, bytes: {}",
            arg,
            arg.chars().count(),
            arg.bytes().len()
        )
    }


}