

// pub fn main() {
//     let mut count = 0;
//     let mut visit = || {
//         count += 1;
//         println!("visit {} times", count);
//     };
//
//     let name = "leo".to_string();
//     let just_call_once = || {
//         let name = name;
//         println!("{}", name);
//     };
//
//     just_call_once();
//     // just_call_once(); // error
//
//     for _ in 0..6 {
//         visit();
//     }
//
//     // call(visit)
// }

fn call_fn<F>(f: F) where F: Fn() { f()
}
fn call_fn_mut<F>(mut f: F) where F: FnMut() { f()
}
fn call_fn_once<F>(f: F) where F: FnOnce() { f()
}
fn call<F>(mut f: F)
    where F: Fn() -> ()
{
    // for _ in 0..6 {
    //     f()
    // }
    f()
}

fn a() {
    let nums:Vec<i32> = (1..11).collect();
    for _ in 1..3 {
        for i in nums.iter().map(|mut v| v * 2) {
            println!("{}", i);
        }
    }
}

pub fn main() {
//     let say_hi = { // forcing the creation of a smaller scope
//         // owned by the smaller scope
//         let name_outer = String::from("Alice");
//         // doesn't work, closure outlives captured values
//         move || {
//             // use by reference
//             let name_inner = &name_outer;
//             println!("Hello, {}", name_inner); }
//     };
// // syntactically invalid, name_outer isn't in this scope //println!("Using name from main: {}", name_outer); // error!
//     say_hi();
//     say_hi();
    let mut name = String::from("Alice");

    let mut say_hi =
        || {
            name += " and Bob";
            println!("Hello, {}", name);

    };
    //call_fn(say_hi);
    call_fn_mut(&mut say_hi);
    call_fn_once(&mut say_hi);
    println!("And now name is: {}", name);

    a();
}
