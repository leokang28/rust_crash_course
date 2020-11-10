fn main() {
    let mut v_4_while = 0;
    let mut v_4_loop = 0;

    // loop
    loop {
        println!("v_4_loop = {}", v_4_loop);
        if v_4_loop > 10 {
            break;
        } else {
            v_4_loop += 1;
        }
    }

    // while
    while v_4_while < 10 {
        println!("v_4_while = {}", v_4_while);
        v_4_while += 1;
    }

    // for
    for i in 0..10 {
        println!("i = {}", i)
    }
}
