use super::mod_math::ModMath;
use primitive_types::U256;
use rand::{prelude::*, rngs::OsRng};

struct GaloisField {
    modulus: U256,
}

impl GaloisField {
    pub fn new(modulus: U256) -> Option<Self> {
      let mod_math = ModMath::new(modulus);
      match Self::is_prime(modulus, 5) {
        true => Some(Self{modulus}),
        false => None
      }
    }

    fn is_prime(modulus: U256, k: usize) -> bool {
        // Miller-Rabin Primality test

        if modulus < U256::from(2) {
            return false;
        }
        if modulus % U256::from(2) == U256::zero() {
            return modulus == U256::from(2);
        }

        let mut d = modulus - U256::one();
        let mut s = 0;

        while d % U256::from(2) == U256::zero() {
            d /= U256::from(2);
            s += 1;
        }

        'outer: for _ in 0..k {
            let a = U256::from(rand::thread_rng().gen_range(2..modulus.as_u128() as u128));
            let mod_math = ModMath::new(modulus);
            // let mut x = mod_exp(U256::from(a), d, n);
            let mut x = mod_math.exp(U256::from(a), d);
            if x == U256::one() || x == modulus - U256::one() {
                continue;
            }
            for _ in 0..s - 1 {
                x = mod_math.exp(x, U256::from(2));
                if x == modulus - U256::one() {
                    continue 'outer;
                }
            }
            return false;
        }
        true
    }
}

