#[cfg(test)]
mod tests {
    use crate::num_mod::NumberUnderMod;
    use primitive_types::U256;
    use crate::num_mod;

    #[test]
    fn test_addition() {
        let num1 = NumberUnderMod::new(10, 13);
        let num2 = NumberUnderMod::new(6, 13);
        let result = num1 + num2;
        assert_eq!(result.unwrap(), NumberUnderMod::new(3, 13));
    }

    #[test]
    fn test_subtraction() {
        let num1 = NumberUnderMod::new(10, 13);
        let num2 = NumberUnderMod::new(6, 13);
        let result = num1 - num2;
        assert_eq!(result.unwrap(), num_mod!(3, 13));
    }

    #[test]
    fn test_multiplication() {
        let num1 = NumberUnderMod::new(10, 13);
        let num2 = NumberUnderMod::new(6, 13);
        let result = num1 * num2;
        assert_eq!(result.unwrap(), num_mod!(8, 13));
    }

    #[test]
    fn test_division() {
        let num1 = NumberUnderMod::new(10, 13);
        let num2 = NumberUnderMod::new(6, 13);
        let result = num1 / num2;
        assert_eq!(result.unwrap(), num_mod!(5, 13));
    }

    #[test]
    fn test_negation() {
        let num = NumberUnderMod::new(10, 13);
        let result = -num;
        assert_eq!(result.unwrap(), num_mod!(3, 13));
    }

    #[test]
    fn test_equality() {
        let num1 = NumberUnderMod::new(10, 13);
        let num2 = NumberUnderMod::new(10, 13);
        assert_eq!(num1, num2);
    }

    #[test]
    fn test_inequality() {
        let num1 = NumberUnderMod::new(10, 13);
        let num2 = NumberUnderMod::new(6, 13);
        assert_ne!(num1, num2);
    }
}