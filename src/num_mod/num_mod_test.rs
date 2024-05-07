
#[cfg(test)]
mod tests {
    use crate::num_mod::{NM};
    use primitive_types::U256;

    #[test]
    fn test_add() {
        // let modulus = U256::from(100);
        let nm_1 = NM::new(101, 101);
        let nm_2 = NM::new(101, 101);
        let sum = nm_1 + nm_2;
        assert_eq!(sum.unwrap(), NM::new(0, 101));
    }

    #[test]
        
    fn test_macro() {
        // let nm_1 = num_mod!(4, 10);
        // assert_eq!(nm_1.value, U256::from(4));
    }
}