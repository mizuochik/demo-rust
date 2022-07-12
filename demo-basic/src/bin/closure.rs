use std::{borrow::BorrowMut, sync::Mutex};

fn foo() {
    println!("hello");
}

fn move_closure() {
    let v = Mutex::new(vec![String::from("foobar")]);
    let _f = || async {
        let mut vv = v.lock().unwrap();
        mutate(vv.borrow_mut());
    };
}

fn mutate(s: &mut Vec<String>) {
    s.push(String::from("hello"));
}

async fn move_async() -> String {
    String::from("hello world")
}

fn call<F: Fn()>(f: F) {
    f();
    f();
}

fn main() {
    let x = vec![100];
    call(move || {
        let y = &x;
        println!("{:?}", y);
    });
}
