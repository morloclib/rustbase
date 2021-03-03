use std::ops;

// id :: forall a . a -> a
pub fn morloc_id<T>(t: T) -> T {
    t
}

// ==================================================================
// Arithmetic
// ==================================================================

pub fn morloc_add<T, U, V>(t: T, u: U) -> V
where
    T: ops::Add<U, Output = V>,
{
    t + u
}

pub fn morloc_sub<T, U, V>(t: T, u: U) -> V
where
    T: ops::Sub<U, Output = V>,
{
    t - u
}

pub fn morloc_mul<T, U, V>(t: T, u: U) -> V
where
    T: ops::Mul<U, Output = V>,
{
    t * u
}

pub fn morloc_div<T, U, V>(t: T, u: U) -> V
where
    T: ops::Div<U, Output = V>,
{
    t / u
}

pub fn morloc_mod(a: i64, b: i64) -> i64 {
    a % b
}

// ==================================================================
// Tuples
// ==================================================================

// forall a b . (a, b) -> a
pub fn morloc_fst<T: Clone, U>(v: &(T, U)) -> T {
    v.0.clone()
}

// forall a b . (a, b) -> a
pub fn morloc_snd<T, U: Clone>(v: &(T, U)) -> U {
    v.1.clone()
}

// forall a b . a -> b -> (a, b)
pub fn morloc_tuple<T: Clone, U: Clone>(t: &T, u: &U) -> (T, U) {
    (t.clone(), u.clone())
}

// forall a b . (a -> b) -> a -> (b, a)
pub fn morloc_couple<T: Clone, U>(f: fn(&T) -> U, t: &T) -> (U, T) {
    (f(&t), t.clone())
}

// forall a b c . (b -> c) -> (a, b) -> (a, c)
pub fn morloc_with_snd<T: Clone, U, V>(f: fn(&U) -> V, v: &(T, U)) -> (T, V) {
    (v.0.clone(), f(&v.1))
}

// forall a b c . (a -> c) -> (a, b) -> (c, b)
pub fn morloc_with_fst<T, U: Clone, V>(f: fn(&T) -> V, v: &(T, U)) -> (V, U) {
    (f(&v.0), v.1.clone())
}

// ==================================================================
// Map functions
// ==================================================================

// forall a b . [(a, b)] -> ([a], [b])
pub fn morloc_unzip<T: Clone, U: Clone>(xs: &[(T, U)]) -> (Vec<T>, Vec<U>) {
    xs.iter().cloned().unzip()
}

// forall a b . (a -> Bool) -> [(a, b)] -> [(a, b)]
pub fn morloc_filter_key<T: Clone, U: Clone>(f: fn(&T) -> bool, xs: &[(T, U)]) -> Vec<(T, U)> {
    xs.iter().filter(|v| f(&v.0)).cloned().collect()
}

// forall a b . (b -> Bool) -> [(a, b)] -> [(a, b)]
pub fn morloc_filter_val<T: Clone, U: Clone>(f: fn(&U) -> bool, xs: &[(T, U)]) -> Vec<(T, U)> {
    xs.iter().filter(|v| f(&v.1)).cloned().collect()
}

// ==================================================================
// Higher order functions
// ==================================================================

// forall a b c . (a -> b -> c) -> [a] -> [b] -> [c]
pub fn morloc_zip_with<T, U, V>(f: fn(&T, &U) -> V, ts: &[T], us: &[U]) -> Vec<V> {
    ts.iter().zip(us.iter()).map(|v| f(v.0, v.1)).collect()
}

// forall a b . (b -> a -> b) -> b -> [a] -> b
pub fn morloc_fold<T: Clone, U>(f: fn(&T, &U) -> T, t: &T, us: &[U]) -> T {
    us.iter().fold(t.clone(), |t, u| f(&t, u))
}

// forall a b . (a -> b) -> [a] -> [b]
pub fn morloc_map<T, U>(f: fn(&T) -> U, xs: &[T]) -> Vec<U> {
    xs.iter().map(f).collect()
}

// forall a . (a -> Bool) -> [a] -> [a]
pub fn morloc_filter<T: Clone>(f: fn(&T) -> bool, xs: &[T]) -> Vec<T> {
    xs.iter().filter(|v| f(*v)).cloned().collect()
}

// ==================================================================
// Lists
// ==================================================================

// forall a . [a] -> a
pub fn morloc_head<T: Clone>(xs: &[T]) -> T {
    xs[0].clone()
}

// forall a . [a] -> [a]
pub fn morloc_tail<T: Clone>(xs: &[T]) -> Vec<T> {
    xs[1..].to_vec()
}

// forall a . [a] -> a
pub fn morloc_last<T: Clone>(xs: &[T]) -> T {
    xs.last().unwrap().clone()
}

// forall a . [a] -> [a]
pub fn morloc_init<T: Clone>(xs: &[T]) -> Vec<T> {
    xs[..xs.len() - 1].to_vec()
}

// forall a . Int -> [a] -> a
pub fn morloc_get<T, U>(i: T, xs: &[U]) -> U
where
    T: std::slice::SliceIndex<[U], Output = U>,
    U: Clone,
{
    xs.get(i).unwrap().clone()
}

// forall a . Int -> [a] -> [a]
pub fn morloc_take<T: Clone>(i: i64, xs: &[T]) -> Vec<T> {
    xs.iter().take(i as usize).cloned().collect()
}

// forall a . Int -> [a] -> [a]
pub fn morloc_drop<T: Clone>(i: i64, xs: &[T]) -> Vec<T> {
    xs.iter().skip(i as usize).cloned().collect()
}

// forall a . (a -> Bool) -> [a] -> [a]
pub fn morloc_take_while<T: Clone>(f: fn(&T) -> bool, xs: &[T]) -> Vec<T> {
    xs.iter().take_while(|v| f(*v)).cloned().collect()
}

// forall a . (a -> Bool) -> [a] -> [a]
pub fn morloc_drop_while<T: Clone>(f: fn(&T) -> bool, xs: &[T]) -> Vec<T> {
    xs.iter().skip_while(|v| f(*v)).cloned().collect()
}

// ==================================================================
// Tests
// ==================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fst() {
        let v = (2, "hewwo");
        assert_eq!(2, morloc_fst(&v));
    }

    #[test]
    fn test_snd() {
        let v = (2, "hewwo");
        assert_eq!("hewwo", morloc_snd(&v));
    }

    #[test]
    fn test_tuple() {
        assert_eq!(
            (2, String::from("hewwo")),
            morloc_tuple(&2, &String::from("hewwo"))
        );
    }

    #[test]
    fn test_couple() {
        assert_eq!(("hewwo", 2), morloc_couple(|_| "hewwo", &2));
    }

    #[test]
    fn test_with_snd() {
        assert_eq!((2, "hewwo"), morloc_with_snd(|_| "hewwo", &(2, 3)));
    }

    #[test]
    fn test_with_fst() {
        assert_eq!(("hewwo", 3), morloc_with_fst(|_| "hewwo", &(2, 3)));
    }

    #[test]
    fn test_head() {
        let v = vec![1, 2, 3];
        assert_eq!(1, morloc_head(&v));
    }

    #[test]
    fn test_tail() {
        let v = vec![1, 2, 3];
        assert_eq!(vec![2, 3], morloc_tail(&v));
    }

    #[test]
    fn test_last() {
        let v = vec![1, 2, 3];
        assert_eq!(3, morloc_last(&v));
    }

    #[test]
    fn test_init() {
        let v = vec![1, 2, 3];
        assert_eq!(vec![1, 2], morloc_init(&v));
    }

    #[test]
    fn test_get() {
        let v = vec![1, 2, 3];
        assert_eq!(2, morloc_get(1, &v));
    }

    #[test]
    fn test_take() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(vec![1, 2, 3], morloc_take(3, &v));
    }

    #[test]
    fn test_drop() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(vec![4, 5], morloc_drop(3, &v));
    }

    #[test]
    fn test_take_while() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(vec![1, 2, 3], morloc_take_while(|v| *v <= 3, &v));
    }

    #[test]
    fn test_drop_while() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(vec![4, 5], morloc_drop_while(|v| *v <= 3, &v));
    }
}
