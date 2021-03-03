use std::ops;

// id :: forall a . a -> a
pub fn morloc_id<T>(t: T) -> T{
    t
}

// ==================================================================
// Arithmetic
// ==================================================================

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
pub fn morloc_fst<T, U>(v: &(T, U)) -> &T {
    &v.0
}

// forall a b . (a, b) -> a
pub fn morloc_snd<T, U>(v: &(T, U)) -> &U {
    &v.1
}

// forall a b . a -> b -> (a, b)
pub fn morloc_tuple<T, U>(t: T, u: U) -> (T, U) {
    (t, u)
}

// forall a b . (a -> b) -> a -> (b, a)
pub fn morloc_couple<T, U>(f: fn(&T) -> U, t: T) -> (U, T) {
    (f(&t), t)
}

// forall a b c . (b -> c) -> (a, b) -> (a, c)
pub fn morloc_with_sec<T, U, V>(f: fn(U) -> V, v: (T, U)) -> (T, V) {
    (v.0, f(v.1))
}

// forall a b c . (a -> c) -> (a, b) -> (c, b)
pub fn morloc_with_fst<T, U, V>(f: fn(T) -> V, v: (T, U)) -> (V, U) {
    (f(v.0), v.1)
}

// ==================================================================
// Higher order functions
// ==================================================================

// forall a b c . (a -> b -> c) -> [a] -> [b] -> [c]
pub fn morloc_zip_with<T, U, V>(f: fn(&T, &U) -> V, ts: &[T], us: &[U]) -> Vec<V> {
    ts.iter().zip(us.iter()).map(|v| f(v.0, v.1)).collect()
}

// forall a b . (b -> a -> b) -> b -> [a] -> b
pub fn morloc_fold<T, U>(f: fn(T, &U) -> T, t: T, us: &[U]) -> T {
    us.iter().fold(t, f)
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

