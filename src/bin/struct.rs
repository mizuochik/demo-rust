struct User {
     name: String,
}

struct Pair(String, i32);

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
    let p = Pair(String::from("foobar"), 100);
    match p {
        Pair(s, _) => println!("{}", s)
    }
}
