

#[cfg(test)]
mod tests {
    use primitive_types::U256;

    use crate::mod_math::{ModMath, IntoU256};


    #[test]
    fn test_add() {
        // let modulus = U256::from(100);
        let modulus = 100;

        let math = ModMath::new(modulus);
        assert_eq!(math.add(U256::from(45), U256::from(60)), U256::from(5));
        assert_eq!(math.add(U256::from(20), U256::from(75)), U256::from(95));
    }

    #[test]
    fn test_sub() {
        let modulus = U256::from(100);
        let math = ModMath::new(modulus);
        assert_eq!(math.sub(U256::from(60), U256::from(45)), U256::from(15));
        assert_eq!(math.sub(U256::from(30), U256::from(40)), U256::from(90));
    }

    #[test]
    fn test_multiply() {
        let modulus = U256::from(100);
        let math = ModMath::new(modulus);
        assert_eq!(math.mul(U256::from(12), U256::from(25)), U256::from(0));
        assert_eq!(math.mul(U256::from(7), U256::from(14)), U256::from(98));
    }

    #[test]
    fn test_exp() {
        let modulus = U256::from(100);
        let math = ModMath::new(modulus);
        assert_eq!(math.exp(U256::from(3), U256::from(4)), U256::from(81));
        assert_eq!(math.exp(U256::from(2), U256::from(8)), U256::from(56));
    }

    #[test]
    fn test_mod_inv() {
        let modulus = U256::from(101);
        let math = ModMath::new(modulus);

        let a = U256::from(10);
        let a_inv = math.inv(a).unwrap();
        assert_eq!(math.mul(a, a_inv), U256::one());

        let b = U256::from(10);
        let b_inv = math.inv(a).unwrap();
        assert_eq!(math.mul(b, b_inv), U256::one());
    }

    #[test]
    fn test_div() {
        let modulus = U256::from(101);
        let math = ModMath::new(modulus);

        let a = U256::from(10);
        let b = U256::from(20);
        assert_eq!(math.div(a, b), U256::from(51));

        let a = U256::from(10);
        let b = U256::from(10);
        assert_eq!(math.div(a, b), U256::one());
    }

    #[test]
    fn test_secp256k1() {
        let p: U256 = U256::from_dec_str("115792089237316195423570985008687907852837564279074904382605163141518161494337").unwrap();
        let math = ModMath::new(p);

        let num = U256::from_dec_str("32670510020758816978083085130507043184471273380659243275938904335757337482424").unwrap();
        let den = U256::from_dec_str("55066263022277343669578718895168534326250603453777594175500187360389116729240").unwrap();

        let den_inv = math.inv(den).unwrap();

        assert_eq!(math.mul(den, den_inv), U256::one());

        assert_eq!(den_inv, U256::from_dec_str("13499648161236477938760301359943791721062504425530739546045302818736391397630").unwrap())

    }

    #[test]
    fn test_into_u256() {
        assert_eq!(10_u32.into_u256(), U256::from(10));
        assert_eq!(10_i32.into_u256(), U256::from(10));
        assert_eq!(10_u64.into_u256(), U256::from(10));
        assert_eq!(10_i64.into_u256(), U256::from(10));
        assert_eq!("10".into_u256(), U256::from(10));
        assert_eq!(U256::from(10).into_u256(), U256::from(10));
    }

    #[test]
    #[should_panic(expected = "Negative value cannot be converted to U256")]
    fn test_into_u256_negative() {
        let _ = (-10_i32).into_u256();
    }

    #[test]
    fn test_square() {
        let math = ModMath::new(100);
        assert_eq!(math.square(10), U256::from(0));
    }

    #[test]
    fn test_sqrt() {
        let math = ModMath::new(113);
        let num = 2;
        let mod_sqrt = math.sqrt(num).unwrap();
        println!("{}", mod_sqrt);
        assert_eq!(math.exp(mod_sqrt, U256::from(2)), U256::from(num));
    }

    // #[test]
    // fn test_big_number_addition() {
    //     let math = ModMath::new(U256::max_value());
    //     let result = math.add(U256::max_value() - U256::from(10), U256::from(5));
    //     assert_eq!(result, U256::from(4));
    // }

    // #[test]
    // fn test_big_number_subtraction() {
    //     let math = ModMath::new(U256::max_value());
    //     let result = math.sub(U256::from(5), U256::max_value() - U256::from(10));
    //     assert_eq!(result, U256::from(6));
    // }

    // #[test]
    // fn test_big_number_multiplication() {
    //     let math = ModMath::new(U256::max_value());
    //     let result = math.mul(U256::max_value() - U256::from(10), U256::from(2));
    //     assert_eq!(result, U256::from(20) - U256::from(1));
    // }

    // #[test]
    // fn test_big_number_division() {
    //     let math = ModMath::new(U256::max_value());
    //     let result = math.div(U256::max_value() - U256::from(10), U256::from(2));
    //     assert_eq!(result, (U256::max_value() - U256::from(10)) / U256::from(2));
    // }

    #[test]
    fn test_big_number_modulus() {
        let math = ModMath::new(U256::max_value());
        let result = math.modulus(U256::max_value() - U256::from(10));
        assert_eq!(result, U256::max_value() - U256::from(10));
    }

    // U256 Tests
    

}
