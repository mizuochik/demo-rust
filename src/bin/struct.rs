struct User {
    name: String,
}

impl User {
    fn hello(&self) {
        println!("{}", self.name)
    }
}

fn main() {
    let u = User {
        name: String::from("foobar"),
    };
    u.hello();
}
