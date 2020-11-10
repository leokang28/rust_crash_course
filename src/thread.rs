use std::thread::{
    sleep,
    spawn,
};
use std::time::Duration;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::cell::RefCell;

pub fn main() {
    let msg = Arc::new(Mutex::new(String::from("Fearless")));
    for _ in 1..11 {
        let msg = msg.clone();
        let inner = move || {
            let mut msg = msg.lock().unwrap();
            msg.push('!');
            println!("{}", msg);
        };
        spawn(inner);
        sleep(Duration::new(1, 0));

    }
}
