struct User {
     name: String,
}

impl User {
    fn hello(&self) {
        println!("hello {}", self.name)
    }
}

fn main() {
    let mut u = User {
        name: String::from("foobar"),
    };
    match u {
        User {name} => println!("{}", name)
    };
    u.name = String::from("hogehoge");
    u.hello();
}
