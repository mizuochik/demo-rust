fn main() {
    let mut x = 100;
    let bx = &mut x;
    *bx = 200;
    println!("{}", x);
}

fn i32_case() {
    let x = 100;
    i_taker(x);
    println!("{}", x);
}

fn string_case() {
    let s = String::from("hello");
    // s_taker(s);
    // println!("{}", s); // couldn't
    s_borrower(&s);
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
