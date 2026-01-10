pub fn abs<T>(num: T) -> T
where
    T: std::ops::Neg<Output = T> + PartialOrd + Copy + num_traits::Zero,
{
    if num < T::zero() {
        return -num;
    }

    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_negative_number_i32() {
        assert_eq!(69, abs(-69));
    }

    #[test]
    fn zero() {
        assert_eq!(0.0, abs(0.0));
    }

    #[test]
    fn positive_number() {
        assert_eq!(69.69, abs(69.69));
    }
}
