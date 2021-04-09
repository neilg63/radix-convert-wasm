#[cfg(test)]
mod tests {

    use crate::conversions::ceil_d128_to_string;
    use crate::conversions::d128_to_f64;
    use crate::conversions::d128_to_parts;
    use crate::conversions::floor_d128_to_string;
    use crate::eval_input;
    use decimal::*;
    use std::str::FromStr;
    #[test]
    fn test_1() {
        assert_eq!(d128!(2.5).to_string(), "2.5".to_string());
    }

    #[test]
    fn test_2() {
        assert_eq!(floor_d128_to_string(d128!(2.5)), "2".to_string());
    }

    #[test]
    fn test_3() {
        assert_eq!(ceil_d128_to_string(d128!(2.5)), "3".to_string());
    }

    #[test]
    fn test_4() {
        let (is_neg, first, second) = d128_to_parts(d128!(-2.333333333333333333));
        assert_eq!(
            (is_neg, first, second),
            (true, "2".to_string(), "333333333333333333".to_string())
        );
    }

    #[test]
    fn test_5() {
        let (is_neg, first, second) = d128_to_parts(d128!(3462.717171717171717171717171));
        assert_eq!(
            (is_neg, first, second),
            (
                false,
                "3462".to_string(),
                "717171717171717171717171".to_string()
            )
        );
    }

    #[test]
    fn test_6() {
        let (is_neg, first, second) = d128_to_parts(d128!(16777216));
        assert_eq!(
            (is_neg, first, second),
            (false, "16777216".to_string(), "".to_string())
        );
    }

    #[test]
    fn test_7() {
        let num_str = "20228262626.2028373636";
        assert_eq!(
            d128::from_str(num_str).ok().unwrap(),
            d128!(20228262626.2028373636)
        );
    }

    //18446744073709552000
    #[test]
    fn test_8() {
        let num_str = "18446744073709551616";
        let (num, _) = eval_input("2 ^ 64".to_string());
        assert_eq!(num.to_string(), num_str.to_string());
    }

    #[test]
    fn test_9() {
        let num_f64 = 18014398509481984_f64;
        let (num, _) = eval_input("2 ^ 54".to_string());
        assert_eq!(num.to_string(), num_f64.to_string());
    }

    #[test]
    fn test_10() {
        let num_f64 = 18014398509481984_f64;
        let num_d128 = d128::from_str(&num_f64.to_string()).ok().unwrap();
        assert_eq!(d128_to_f64(num_d128), num_f64);
    }

    #[test]
    fn test_11() {
        let num_f64 = 0.142857142857142857_f64;
        let num_d128 = d128::from_str(&num_f64.to_string()).ok().unwrap();
        assert_eq!(d128_to_f64(num_d128), num_f64);
    }

    #[test]
    fn test_12() {
        let num_str = "0.142857142857142857";
        let (num, _) = eval_input(num_str.to_string());
        assert_eq!(num.to_string(), num_str.to_string());
    }

    #[test]
    fn test_13() {
        let (result, _) = eval_input("1mi + 100m".to_string());
        assert_eq!("1709.344", result.to_string());
    }

    #[test]
    fn test_14() {
        let (result, _) = eval_input("5 feet + 0mm".to_string());
        assert_eq!("1524.0", result.to_string());
    }

    #[test]
    fn test_15() {
        let (result, unit) = eval_input("5 feet + 2in".to_string());
        let response = format!("{}{}", result, unit);
        assert_eq!("62in", response);
    }
}
