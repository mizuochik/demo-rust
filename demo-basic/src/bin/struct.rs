#[derive(Debug)]
pub enum Foo {
    Foo,
    Bar,
}

#[derive(Debug)]
pub enum Bar {
    Foo,
    Bar,
}

#[derive(Debug)]
pub struct ST<F>
 {
    pub items: Vec<F>,
}

fn main() {
    let v = ST{
        items: vec![Foo::Foo],
    };
    println!("{:?}", v);
}
