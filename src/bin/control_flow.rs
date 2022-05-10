fn main() {
    let n= 3;
    if n > 0 {
        println!("> 0");
    } else {
        println!("<= 0");
    }

    let m = 3;
    let n = if m > 0 { 3 } else { 0 };
    println!("{}", n);
}
