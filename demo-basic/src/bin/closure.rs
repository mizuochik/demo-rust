use std::{sync::Mutex, borrow::BorrowMut};

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

#[tokio::main]
async fn main() {
    let _s = move_async();

    move_async().await;
}
