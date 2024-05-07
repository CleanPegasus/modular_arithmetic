
use crate::mod_math::IntoU256;

use super::mod_math::ModMath;
use primitive_types::U256;
use rand::{prelude::*, rngs::OsRng};
use std::ops::{Add, Mul, Sub, Div, Neg};
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct NumberUnderMod {
    value: U256,
    modulus: U256,
}

impl NumberUnderMod {
    pub fn new<T: IntoU256>(value: T, modulus: T) -> Self {
      let value = value.into_u256();
      let modulus = modulus.into_u256();
      Self {
        value: value % modulus,
        modulus
      }
    }
}

impl Add for NumberUnderMod {
  type Output = Result<Self, &'static str>;

  fn add(self, other: Self) -> Self::Output {
      if self.modulus != other.modulus {
          Err("Cannot add numbers with different moduli")
      } else {
          let math = ModMath::new(self.modulus);
          Ok(NumberUnderMod {
              value: math.add(self.value, other.value),
              modulus: self.modulus,
          })
      }
  }
}

impl Mul for NumberUnderMod {
  type Output = Result<Self, &'static str>;

  fn mul(self, other: Self) -> Self::Output {
      if self.modulus != other.modulus {
          Err("Cannot add numbers with different moduli")
      } else {
          let math = ModMath::new(self.modulus);
          Ok(NumberUnderMod {
              value: math.mul(self.value, other.value),
              modulus: self.modulus,
          })
      }
  }
}

impl Sub for NumberUnderMod {
  type Output = Result<Self, &'static str>;

  fn sub(self, other: Self) -> Self::Output {
      if self.modulus != other.modulus {
          Err("Cannot add numbers with different moduli")
      } else {
          let math = ModMath::new(self.modulus);
          Ok(NumberUnderMod {
              value: math.sub(self.value, other.value),
              modulus: self.modulus,
          })
      }
  }
}

impl Div for NumberUnderMod {
  type Output = Result<Self, &'static str>;

  fn div(self, other: Self) -> Self::Output {
      if self.modulus != other.modulus {
          Err("Cannot add numbers with different moduli")
      } else {
          let math = ModMath::new(self.modulus);
          Ok(NumberUnderMod {
              value: math.div(self.value, other.value),
              modulus: self.modulus,
          })
      }
  }
}

impl Neg for NumberUnderMod {
  type Output = Result<Self, &'static str>;

  fn neg(self) -> Self::Output {
    let math = ModMath::new(self.modulus);
    Ok(NumberUnderMod {
      value: math.add_inv(self.value),
      modulus: self.modulus,
  })
  }
}

impl PartialEq for NumberUnderMod {
  fn eq(&self, other: &NumberUnderMod) -> bool {
    self.value == other.value && self.modulus == other.modulus
  }
}