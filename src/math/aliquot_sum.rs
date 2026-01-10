pub fn aliquot_sum(number: u64) -> u64 {
    if number == 0 {
        panic!("Input has to be positive.")
    }

    (1..=number / 2).filter(|&d| number.is_multiple_of(d)).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_aliquot_sum {
        ($($name:ident: $tc:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (number, expected) = $tc;
                assert_eq!(aliquot_sum(number), expected);
            }
        )*
        }
    }

    test_aliquot_sum! {
        zero_case_is_not_tested_here: (1, 0),
        perfect_six: (6, 6),
        deficient_eight: (8, 7),
    }
}
