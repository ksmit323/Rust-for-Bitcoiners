#![allow(unused)]

use core::fmt;

enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        matches!(*self, MResult::Ok(_)) // implementation is straight out of the Result docs
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        matches!(*self, MResult::Err(_))
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T
    where
        E: fmt::Debug,
    {
        match self {
            MResult::Ok(t) => t,
            MResult::Err(e) => panic!("{e:?}"),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E
    where
        T: fmt::Debug,
    {
        match self {
            MResult::Ok(t) => panic!("{t:?}"),
            MResult::Err(e) => e,
        }
    }
}

// Add unit tests below
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let result: MResult<i8, &str> = MResult::ok(1);
        assert!(matches!(result, MResult::Ok(1)));
    }

    #[test]
    fn test_err() {
        let result: MResult<i8, &str> = MResult::err("Error!");
        assert!(matches!(result, MResult::Err("Error!")));
    }

    #[test]
    fn test_is_ok() {
        let ok_result: MResult<i8, &str> = MResult::ok(1);
        let err_result: MResult<i8, &str> = MResult::err("Error!");
        assert!(ok_result.is_ok());
        assert!(!err_result.is_ok());
    }

    #[test]
    fn test_is_err() {
        let ok_result: MResult<i8, &str> = MResult::ok(1);
        let err_result: MResult<i8, &str> = MResult::err("Error!");
        assert!(!ok_result.is_err());
        assert!(err_result.is_err());
    }

    #[test]
    fn test_unwrap() {
        let ok_result: MResult<i8, &str> = MResult::ok(1);
        assert_eq!(ok_result.unwrap(), 1);
    }

    #[test]
    fn test_unwrap_err() {
        let err_result: MResult<i8, &str> = MResult::err("Error!");
        assert_eq!(err_result.unwrap_err(), "Error!");
    }
}
