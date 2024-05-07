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
    pub fn new<T: IntoU256>(modulus: T) -> Self {
        let modulus = modulus.into_u256();
        if modulus == U256::zero() {
            panic!("Modulus Cannot be Zero");
        }
        ModMath {
            modulus
        }
    }

    pub fn modulus<T: IntoU256>(&self, a: T) -> U256 {
        a.into_u256() % self.modulus
    }

    /// Adds two `U256` numbers under the modulus.
    pub fn add<T: IntoU256>(&self, a: T, b: T) -> U256 {
        let a = a.into_u256();
        let b = b.into_u256();
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
    pub fn sub<T: IntoU256>(&self, a: T, b: T) -> U256 {
        let a = a.into_u256();
        let b = b.into_u256();
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
    pub fn mul<T: IntoU256>(&self, a: T, b: T) -> U256 {
        let a_mod = a.into_u256() % self.modulus;
        let b_mod = b.into_u256() % self.modulus;
    
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
    pub fn exp<T: IntoU256>(&self, base: T, exponent: T) -> U256 {
        let mut result = U256::one();
        let mut base = base.into_u256() % self.modulus;
        let mut exponent = exponent.into_u256();
        while exponent != U256::zero() {
            if exponent % U256::from(2) != U256::zero() {
                result = self.mul(result, base)
            }
            base = self.square(base);
            exponent = exponent / U256::from(2);
        }
        result
    }

    /// Calculates the modular multiplicative inverse of a `U256` number under the modulus.
    ///
    /// Returns `None` if the inverse does not exist.
    pub fn inv<T: IntoU256>(&self, a: T) -> Option<U256> {
        let (mut m, mut x0, mut x1) = (self.modulus, U256::zero(), U256::one());
        let mut a = a.into_u256() % self.modulus;
        if self.modulus == U256::one() {
            return None;
        }
    
        while a > U256::one() {
            let q = a / m;
            let mut temp = m;
    
            m = a % m;
            a = temp;
            temp = x0;
            let t = self.mul(q, x0);
            x0 = self.sub(x1, t);
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
    pub fn div<T: IntoU256>(&self, a: T, b: T) -> U256 {
        let b = b.into_u256();
        let b_inv = self.inv(b).unwrap_or_else(|| {
            panic!("Cannot find Inverse of {}", b);
        });
         self.mul(a.into_u256(), b_inv)
    }

    pub fn add_inv<T: IntoU256>(&self, a: T) -> U256 {
      // self.modulus - a.into_u256()
      let a = a.into_u256();
      if a == U256::zero() {
        U256::zero()
      } else {
        self.modulus - a
      }
    }
    
    /// Checks if two `U256` numbers are equivalent under the modulus.
    pub fn eq<T: IntoU256>(&self, a: T, b: T) -> bool {
        a.into_u256() % self.modulus == b.into_u256() % self.modulus
    }

    /// Squares a given U256 number under modulus
    pub fn square<T: IntoU256>(&self, a: T) -> U256 {
        let a = a.into_u256();
        self.mul(a, a)
    }

    fn u512_to_u256(result: U512) -> U256 {
        let mut result_little_endian = [0_u8; 64];
        result.to_little_endian(&mut result_little_endian);
        U256::from_little_endian(&result_little_endian[..32])
    }

    pub fn sqrt(&self, a: U256) -> U256 {
        unimplemented!()
    }

}


pub trait IntoU256 {
    fn into_u256(self) -> U256;
}

impl IntoU256 for u32 {
    fn into_u256(self) -> U256 {
        U256::from(self)
    }
}

impl IntoU256 for i32 {
    fn into_u256(self) -> U256 {
        if self < 0 {
            panic!("Negative value cannot be converted to U256");
        }
        U256::from(self as u32)  // Safe cast since the value is non-negative
    }
}

impl IntoU256 for u64 {
    fn into_u256(self) -> U256 {
        U256::from(self)
    }
}

impl IntoU256 for i64 {
    fn into_u256(self) -> U256 {
        if self < 0 {
            panic!("Negative value cannot be converted to U256");
        }
        U256::from(self as u64)  // Safe cast since the value is non-negative
    }
}

impl IntoU256 for &str {
    fn into_u256(self) -> U256 {
        U256::from_dec_str(self).unwrap()
    }
}

impl IntoU256 for U256 {
    fn into_u256(self) -> U256 {
        self
    }
}