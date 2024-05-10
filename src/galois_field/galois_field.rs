use crate::mod_math::{ModMath, IntoU256};
use crate::num_mod::{NumberUnderMod as NM};
use primitive_types::U256;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub struct GaloisField {
    modulus: U256,
}

impl GaloisField {

    pub fn new<T: IntoU256>(modulus: T) -> Option<Self> { // TODO: Change to Result<Self, Err>
        let modulus = modulus.into_u256();
        if Self::is_valid_galois_field_size(modulus) {
            return Some(Self { modulus });
        } else {
            None
        }
    }

    pub fn gf(&self, value: U256) -> NM {
        NM::new(value, self.modulus)
    }

    fn prime_factors(mut n: U256) -> HashMap<U256, U256> {
        let mut factors = HashMap::new();
        let mut count: U256;

        count = U256::zero();
        while n % U256::from(2) == U256::zero() {
            count += U256::one();
            n /= U256::from(2);
        }
        if count > U256::zero() {
            factors.insert(U256::from(2), count);
        }

        let mut i = U256::from(3);
        while i * i <= n {
            count = U256::zero();
            while n % i == U256::zero() {
                count += U256::one();
                n /= i;
            }
            if count > U256::zero() {
                factors.insert(i, count);
            }
            i += U256::from(2);
        }

        if n > U256::from(2) {
            factors.insert(n, U256::from(1));
        }
    
        factors
    }
    
    fn is_valid_galois_field_size(n: U256) -> bool {
        let factors = Self::prime_factors(n);
        factors.len() == 1 && factors.values().all(|&count| count >= U256::from(1))
    }
}

pub enum GaloisFieldError {
    InvalidModulus
}