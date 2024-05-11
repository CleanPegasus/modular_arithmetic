
use crate::mod_math::{IntoU256, ModMath};

use primitive_types::U256;
use std::ops::{Add, Mul, Sub, Div, Neg};
use std::cmp::PartialEq;
/// `NumberUnderMod` represents a number under a certain modulus.
///
/// This struct provides methods for performing arithmetic operations
/// (addition, subtraction, multiplication, division, and negation)
/// under the modulus.
///
/// # Examples
///
/// let num1 = NumberUnderMod::new(5, 7);
/// let num2 = NumberUnderMod::new(3, 7);
/// let result = num1 + num2;
/// assert_eq!(result.unwrap().value, 1);
#[derive(Debug)]
pub struct NumberUnderMod {
    value: U256,
    modulus: U256,
}

/// Creates a new `NumberUnderMod` with the given value and modulus.
    ///
    /// The value is automatically reduced modulo the modulus.
    ///
    /// # Examples
    ///
    /// ```
    /// let num = NumberUnderMod::new(10, 7);
    /// assert_eq!(num.value, 3);
    /// ```
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

/// num_mod is a convenience macro for creating a new NumberUnderMod instance. 
/// # Arguments 
/// * $value - The value of the number.
/// * $modulus - The modulus under which the number is considered. 
/// # Examples 
/// let num = num_mod!(10, 6); 
/// assert_eq!(num.value(), 4); 
/// This example creates a new NumberUnderMod with a value of 10 under modulus 6. 
/// The value is automatically reduced modulo 6, so the resulting NumberUnderMod has a value of 4.

#[macro_export]
macro_rules! num_mod {
    ($value:expr, $modulus:expr) => {
        NumberUnderMod::new($value, $modulus)
    };
}

