pub trait Database {
    fn select(&self) -> String;
}

pub trait DatabaseDi {
    type Database: Database;
    fn database(&self) -> &Self::Database;
}

pub trait DatabaseImpl {}

impl<T: DatabaseImpl> Database for T {
    fn select(&self) -> String {
        String::from("<selected>")
    }
}

pub trait UseCase {
    fn run(&self) -> String;
}

pub trait UseCaseDi {
    type UseCase: UseCase;
    fn use_case(&self) -> &Self::UseCase;
}

pub trait UseCaseImpl: DatabaseDi {}

impl<T: UseCaseImpl> UseCase for T {
    fn run(&self) -> String {
        self.database().select()
    }
}

pub trait Handler {
    fn handle(&self);
}

pub trait HandlerDi {
    type Handler: Handler;
    fn handler(&self) -> &Self::Handler;
}

pub trait HandlerImpl: UseCaseDi {}

impl<T: HandlerImpl> Handler for T {
    fn handle(&self) {
        println!("{}", self.use_case().run());
    }
}

pub struct Di {}

pub fn new_di() -> Di {
    Di {}
}
impl DatabaseImpl for Di {}
impl UseCaseImpl for Di {}
impl HandlerImpl for Di {}
impl DatabaseDi for Di {
    type Database = Self;
    fn database(&self) -> &Self::Database {
        self
    }
}
impl UseCaseDi for Di {
    type UseCase = Self;
    fn use_case(&self) -> &Self::UseCase {
        self
    }
}
impl HandlerDi for Di {
    type Handler = Self;
    fn handler(&self) -> &Self::Handler {
        self
    }
}

fn main() {
    new_di().handler().handle();
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mock {
        message: String,
    }
    impl Database for Mock {
        fn select(&self) -> String {
            self.message.clone()
        }
    }
    impl DatabaseDi for Mock {
        type Database = Self;
        fn database(&self) -> &Self::Database {
            self
        }
    }
    impl UseCaseImpl for Mock {}
    impl UseCaseDi for Mock {
        type UseCase = Self;
        fn use_case(&self) -> &Self::UseCase {
            self
        }
    }

    #[test]
    fn use_case_run() {
        let m = Mock {
            message: String::from("<mock>"),
        };
        assert_eq!(String::from("<mock>"), m.use_case().run());
    }
}
