use std::ops;

// ==================================================================
// Arithmetic
// ==================================================================

// id :: forall a . a -> a
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

// ==================================================================
// Tuples
// ==================================================================

// forall a b . (a, b) -> a
pub fn fst<T, U>(v: &(T, U)) -> &T {
    &v.0
}

// forall a b . (a, b) -> a
pub fn snd<T, U>(v: &(T, U)) -> &U {
    &v.1
}

// forall a b . a -> b -> (a, b)
pub fn tuple<T, U>(t: T, u: U) -> (T, U) {
    (t, u)
}

// forall a b . (a -> b) -> a -> (b, a)
pub fn couple<T, U>(f: fn(&T) -> U, t: T) -> (U, T) {
    (f(&t), t)
}

// forall a b c . (b -> c) -> (a, b) -> (a, c)
pub fn with_sec<T, U, V>(f: fn(&U) -> V, v: (T, U)) -> (T, V) {
    (v.0, f(&v.1))
}

// forall a b c . (a -> c) -> (a, b) -> (c, b)
pub fn with_fst<T, U, V>(f: fn(&T) -> V, v: (T, U)) -> (V, U) {
    (f(&v.0), v.1)
}

// ==================================================================
// Lists
// ==================================================================

// forall a . [a] -> a
pub fn morloc_head<T>(xs: &[T]) -> &T {
    &xs[0]
}

// forall a . [a] -> [a]
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

