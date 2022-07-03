fn main() {
    let mut s = String::from("hello");
    borrower(&s);
    mat_borrower(&mut s);
    mover(s);
}

fn borrower(s: &String) {
    println!("{}", s);
}

fn mat_borrower(s: &mut String) {
    println!("{}", s)
}

fn mover(s: String) {
    println!("{}", s);
}

