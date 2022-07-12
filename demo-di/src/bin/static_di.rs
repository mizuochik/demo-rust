use std::sync::Arc;

#[cfg_attr(test, mockall::automock)]
pub trait Database {
    fn select(&self) -> String;
}

pub struct DatabaseImpl {}

impl Database for DatabaseImpl {
    fn select(&self) -> String {
        String::from("selected")
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait UseCase {
    fn run(&self) -> String;
}

pub struct UseCaseImpl<D> {
    pub database: Arc<D>,
}

impl<D> UseCase for UseCaseImpl<D>
where
    D: Database,
{
    fn run(&self) -> String {
        self.database.select()
    }
}

pub struct Handler<U> {
    pub use_case: Arc<U>,
}

impl<U> Handler<U>
where
    U: UseCase,
{
    fn handle(&self) {
        println!("run: {}", self.use_case.run());
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
            database: Arc::new(self.database()),
        }
    }

    pub fn handler(&self) -> Handler<impl UseCase> {
        Handler {
            use_case: Arc::new(self.use_case()),
        }
    }
}

fn main() {
    let d = Di {};
    d.handler().handle();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_case() {
        let db = Arc::new({
            let mut db = MockDatabase::new();
            db.expect_select().times(1).return_const("<mocked>");
            db
        });
        let uc = UseCaseImpl { database: db };
        assert_eq!("<mocked>", uc.run());
    }
}
