use std::{rc::Rc, sync::Arc, thread};

/// Smart Pointer

pub fn box_example() {
    let a = Box::new(5);
    println!("a: {}", a);

    let b = *a + 1;
}

pub fn rc_example() {
    let s = String::from("Hello");
    let rc = Rc::new(s);
    let rc2 = Rc::clone(&rc);

    assert_eq!(Rc::strong_count(&rc2), 2);

    let arc_s = Arc::new(String::from("Arc Test"));
    for _ in 0..10 {
        let s = Arc::clone(&arc_s);
        let handle = thread::spawn(move || {
           println!("s: {}", s);
        });
        handle.join().unwrap();
    }
}

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        box_example();
        rc_example();
    }
}
