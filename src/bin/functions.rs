fn main() {
    let x = 100;
    let y = twice(&x);
    println!("x={} y={}", x, y);
}

fn print(message: &i32) {
    println!("{}", message);
}

fn twice(x : &i32) -> i32 {
    x * 2
}
