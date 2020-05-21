pub enum Mistake<T, E> {
    Fine(T, Vec<E>),
    Fail(Vec<E>),
}

impl<T, E> Mistake<T, E> {
    pub fn to_option(self, errors: &mut Vec<E>) -> Option<T> {
        match self {
            Mistake::Fine(val, errs) => {
                errors.extend(errs);
                Some(val)
            }
            Mistake::Fail(errs) => {
                errors.extend(errs);
                None
            }
        }
    }
}

impl<T, E> From<Result<T, E>> for Mistake<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(val) => Mistake::Fine(val, Vec::new()),
            Err(err) => Mistake::Fail(vec![err]),
        }
    }
}

#[macro_export]
macro_rules! attempt {
    ($mistake:expr, $errors:expr) => {
        match $mistake.to_option(&mut $errors) {
            Some(val) => val,
            None => return crate::Mistake::Fail($errors),
        }
    };
}

#[macro_export]
macro_rules! attempt_res {
    ($result:expr, $errors:expr) => {
        match $crate::Mistake::from($result).to_option(&mut $errors) {
            Some(val) => val,
            None => return crate::Mistake::Fail($errors),
        }
    };
}
