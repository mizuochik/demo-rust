struct User {
    name: String,
}

fn main() {
    let mut u = User {
        name: String::from("foobar"),
    };
    u.name = String::from("hogefuga");
    println!("{}", u.name);
}
