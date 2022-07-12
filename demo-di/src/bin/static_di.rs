pub trait Database {
    fn select(&self) -> String;
}

pub struct DatabaseImpl {}

impl Database for DatabaseImpl {
    fn select(&self) -> String {
        String::from("selected")
    }
}

pub trait UseCase {
    fn run(&self);
}

pub struct UseCaseImpl<D> {
    pub database: D,
}

impl<D> UseCase for UseCaseImpl<D>
where
    D: Database,
{
    fn run(&self) {
        println!("selected: {}", self.database.select());
    }
}

pub trait Handler {
    fn handle(&self);
}

pub struct HandlerImpl<U> {
    pub use_case: U,
}

impl<U> Handler for HandlerImpl<U>
where
    U: UseCase,
{
    fn handle(&self) {
        self.use_case.run();
    }
}

pub struct Di {}

impl Di {
    pub fn new() -> Self {
        Di {}
    }

    pub fn database(&self) -> impl Database {
        DatabaseImpl {}
    }

    pub fn use_case(&self) -> impl UseCase {
        UseCaseImpl {
            database: self.database(),
        }
    }

    pub fn handler(&self) -> impl Handler {
        HandlerImpl {
            use_case: self.use_case(),
        }
    }
}

fn main() {
    let d = Di {};
    d.handler().handle();
}
