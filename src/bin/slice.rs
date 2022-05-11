fn main() {
    let b = "hogehoge".as_bytes();
    let bv = b[0];
    let bs = &b[0..2];
    println!("{}", bv);
    println!("{}", bs[0]);
    iter(b);
}

fn iter(b: &[u8]) {
    for v in b.iter() {
        println!("{}", v);
    }
}
