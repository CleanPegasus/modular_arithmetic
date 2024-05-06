use primitive_types::{U256, U512};

/// `ModMath` is a struct that provides modular arithmetic operations.
///
/// It operates on unsigned 256-bit integers (`U256`) and performs operations under a given modulus.
/// The modulus is provided when creating a new `ModMath` instance and cannot be zero.
pub struct ModMath {
    modulus: U256,
}

impl ModMath {
    // TODO: Take generic modulus and convert it to U256
    /// Creates a new `ModMath` instance with the given modulus.
    ///
    /// # Panics
    ///
    /// Panics if the modulus is zero.
    pub fn new(modulus: U256) -> Self {
        if modulus == U256::zero() {
            panic!("Modulus Cannot be Zero");
        }
        ModMath {
            modulus
        }
    }

    pub fn modulus(&self, a: U256) -> U256 {
        a % self.modulus
    }

    /// Adds two `U256` numbers under the modulus.
    pub fn add(&self, a: U256, b: U256) -> U256 {
        // (a % self.modulus + b % self.modulus) % self.modulus
        match a.checked_add(b) {
            Some(sum) => sum % self.modulus,
            None => {
                let a_512 = U512::from(a);
                let b_512 = U512::from(b);
                let modulus_512 = U512::from(self.modulus);
                let result = (a_512 + b_512) % modulus_512;

                ModMath::u512_to_u256(result)
            }
        }
    }

    /// Subtracts the second `U256` number from the first one under the modulus.
    pub fn sub(&self, a: U256, b: U256) -> U256 {
        if b > a {
            // (self.modulus + a - b) % self.modulus
            match self.modulus.checked_add(a) {
                Some(sum) => (sum - b) % self.modulus,
                None => {
                    let a_512 = U512::from(a);
                    let b_512 = U512::from(b);
                    let modulus_512 = U512::from(self.modulus);
                    let result = (modulus_512 + a_512 - b_512) % modulus_512;

                    ModMath::u512_to_u256(result)
                }
            }
        } else {
            (a - b) % self.modulus
        }
    }

    /// Multiplies two `U256` numbers under the modulus.
    pub fn mul(&self, a: U256, b: U256) -> U256 {
        let a_mod = a % self.modulus;
        let b_mod = b % self.modulus;
    
        // Use checked_mul for safe multiplication
        match a_mod.checked_mul(b_mod) {
            Some(product) => product % self.modulus,
            None => {
                let a_mod_u512 = U512::from(a_mod);
                let b_mod_u512 = U512::from(b_mod);
                let result  = a_mod_u512 * b_mod_u512 % U512::from(self.modulus);

                ModMath::u512_to_u256(result)
            },
        }
    }
    

    /// Raises the base to the power of the exponent under the modulus.
    pub fn exp(&self, base: U256, exponent: U256) -> U256 {
        let mut result = U256::one();
        let mut base = base % self.modulus;
        let mut exponent = exponent;
        while exponent != U256::zero() {
            if exponent % U256::from(2) != U256::zero() {
                result = (result * base) % self.modulus;
            }
            base = (base * base) % self.modulus;
            exponent = exponent / U256::from(2);
        }
        result
    }

    /// Calculates the modular multiplicative inverse of a `U256` number under the modulus.
    ///
    /// Returns `None` if the inverse does not exist.
    pub fn inv(&self, a: U256) -> Option<U256> {
        let (mut m, mut x0, mut x1) = (self.modulus, U256::zero(), U256::one());
        let mut a = a % self.modulus;
        if self.modulus == U256::one() {
            return None;
        }
    
        while a > U256::one() {
            let q = a / m;
            println!("Entered here");
            let mut temp = m;
    
            m = a % m;
            a = temp;
            dbg!(a);
            temp = x0;
            // let t = (q * x0) % self.modulus;
            let t = self.mul(q, x0);
            dbg!(t);
            x0 = self.sub(x1, t);
            dbg!(x0);
            x1 = temp;
        }
    
        if x1 < U256::zero() {
            x1 = self.add(x1, self.modulus);
        }
    
        if a != U256::one() {
            None
        } else {
            Some(x1)
        }
    }

    /// Divides the first `U256` number by the second one under the modulus.
    ///
    /// # Panics
    ///
    /// Panics if the second number is zero or if its inverse does not exist under the modulus.
    pub fn div(&self, a: U256, b: U256) -> U256 {
        let b_inv = self.inv(b).unwrap_or_else(|| {
            panic!("Cannot find Inverse of {}", b);
        });
         self.mul(a, b_inv)
    }
    
    /// Checks if two `U256` numbers are equivalent under the modulus.
    pub fn eq(&self, a: U256, b: U256) -> bool {
        a % self.modulus == b % self.modulus
    }

    /// Squares a given U256 number under modulus
    pub fn square(&self, a: U256) -> U256 {
        self.mul(a, a)
    }

    fn u512_to_u256(result: U512) -> U256 {
        let mut result_little_endian = [0_u8; 64];
        result.to_little_endian(&mut result_little_endian);
        U256::from_little_endian(&result_little_endian[..32])
    }

    // pub fn sqrt(&self, a: U256) -> U256 {}

}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive_types::U256;

    #[test]
    fn test_add() {
        let modulus = U256::from(100);

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

        // let slope = math.mul(num, den_inv);
        // let slope = math.div(num, den);
        
        // println!("{}", slope);
        // assert!(slope);
    }
}
