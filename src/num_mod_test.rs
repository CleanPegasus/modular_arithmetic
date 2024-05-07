
#[cfg(test)]
mod tests {
    use crate::num_mod::NumberUnderMod as NM;

    #[test]
    fn test_add() {
        // let modulus = U256::from(100);
        let nm_1 = NM::new(101, 101);
        let nm_2 = NM::new(101, 101);

        assert_eq!((nm_1 + nm_2).unwrap(), NM::new(0, 101));
    }
}