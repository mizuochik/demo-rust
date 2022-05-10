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

    let a = [0, 1, 2];
    for av in a {
        println!("{}", av);
    }

    let mut x = 3;
    let y = loop {
        x += 1;
        if x > 3 {
            break x;
        }
    };
    println!("{}", y);
}
