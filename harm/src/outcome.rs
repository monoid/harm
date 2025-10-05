/* Copyright (C) 2025 Ivan Boldyrev
 *
 * This document is licensed under the BSD 3-clause license.
 */

pub trait Outcome {
    type Inner;
    type Output<Y>;

    fn map<Y>(self, op: impl FnOnce(Self::Inner) -> Y) -> Self::Output<Y>;
}

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Unfallible<X>(pub X);

impl<X> Outcome for Unfallible<X> {
    type Inner = X;
    type Output<Y> = Y;

    fn map<Y>(self, op: impl FnOnce(X) -> Y) -> Self::Output<Y> {
        op(self.0)
    }
}

impl<X, E> Outcome for Result<X, E> {
    type Inner = X;
    type Output<Y> = Result<Y, E>;

    fn map<Y>(self, op: impl FnOnce(X) -> Y) -> Self::Output<Y> {
        Result::map(self, op)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfallible() {
        let u = Unfallible(42);
        let y = Outcome::map(u, |x| x + 1);

        // comparison should work only for i32...
        assert_eq!(y, 43);
    }

    #[test]
    fn test_result_ok() {
        let r: Result<i32, &'static str> = Ok(42);
        let y = Outcome::map(r, |x| x + 1);
        assert_eq!(y, Ok(43));
    }

    #[test]
    fn test_result_err() {
        let r: Result<i32, &'static str> = Err("error");
        let y = Outcome::map(r, |x| x + 1);
        assert_eq!(y, Err("error"));
    }
}
