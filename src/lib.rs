use std::ops;

pub fn morloc_id<T>(t: T) -> T{
    t
}

pub fn morloc_add<T, U, V>(t: T, u: U) -> V
    where T: ops::Add<U, Output=V> {
    t + u
}

pub fn morloc_sub<T, U, V>(t: T, u: U) -> V
    where T: ops::Sub<U, Output=V> {
    t - u
}

pub fn morloc_mul<T, U, V>(t: T, u: U) -> V
    where T: ops::Mul<U, Output=V> {
    t * u
}

pub fn morloc_div<T, U, V>(t: T, u: U) -> V
    where T: ops::Div<U, Output=V> {
    t / u
}

pub fn morloc_mod(a: i64, b: i64) -> i64 {
    a % b
}

pub fn morloc_head<T>(xs: &[T]) -> &T {
    &xs[0]
}

pub fn morloc_tail<T>(xs: &[T]) -> &[T] {
    &xs[1..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head() {
        let v = vec![1, 2, 3];
        assert_eq!(1, *morloc_head(&v));
    }

    #[test]
    fn test_tail() {
        let v = vec![1, 2, 3];
        assert_eq!([2, 3], *morloc_tail(&v));
    }
}

