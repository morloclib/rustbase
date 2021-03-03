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

