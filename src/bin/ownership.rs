fn main() {
    let mut x = 100;
    let bx = &mut x;
    *bx = 200;
    println!("{}", x);
    i32_case();
    string_case();
}

fn i32_case() {
    let x = 100;
    i_taker(x);
    println!("{}", x);
}

fn string_case() {
    let mut s = String::from("hello");
    // s_taker(s);
    // println!("{}", s); // couldn't
    s_borrower(&s);
    println!("{}", s);
    s_mut_borrower(&mut s);
    println!("{}", s);
}

fn i_taker(x: i32){
    println!("{}", x);
}

fn s_taker(s: String) {
    println!("{}", s)
}

fn s_borrower(s: &String) {
    println!("{}", s)
}

fn s_mut_borrower(s: &mut String) {
    *s = String::from("bye");
}
