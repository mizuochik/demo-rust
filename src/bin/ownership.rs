fn main() {
    let mut x = 100;
    let bx = &mut x;
    *bx = 200;
    println!("{}", x);
}
