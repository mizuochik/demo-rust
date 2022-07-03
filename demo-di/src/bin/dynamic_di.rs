use std::sync::Arc;

pub trait Database {
    fn select(&self) -> String;
}

pub struct DatabaseImpl {}

impl Database for DatabaseImpl {
    fn select(&self) -> String {
        String::from("<selected>")
    }
}

pub trait UseCase {
    fn run(&self) -> String;
}

pub struct UseCaseImpl {
    database: Arc<dyn Database>,
}

impl UseCase for UseCaseImpl {
    fn run(&self) -> String {
        self.database.select()
    }
}

pub trait Handler {
    fn handle(&self);
}

pub struct HandlerImpl {
    use_case: Arc<dyn UseCase>,
}

impl Handler for HandlerImpl {
    fn handle(&self) {
        println!("{}", self.use_case.run());
    }
}

pub struct Di {
    pub database: Arc<dyn Database>,
    pub use_case: Arc<dyn UseCase>,
    pub handler: Arc<dyn Handler>,
}

pub fn new_di() -> Di {
    let db = Arc::new(DatabaseImpl {});
    let uc = Arc::new(UseCaseImpl {
        database: db.clone(),
    });
    let h = Arc::new(HandlerImpl {
        use_case: uc.clone(),
    });
    Di {
        database: db.clone(),
        use_case: uc.clone(),
        handler: h.clone(),
    }
}

fn main() {
    new_di().handler.handle();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DatabaseMock {
        message: String,
    }
    impl Database for DatabaseMock {
        fn select(&self) -> String {
            self.message.clone()
        }
    }

    #[test]
    fn use_case_run() {
        let db = Arc::new(DatabaseMock {
            message: String::from("<mock>"),
        });
        let uc = UseCaseImpl { database: db };
        assert_eq!("<mock>", uc.run());
    }
}
