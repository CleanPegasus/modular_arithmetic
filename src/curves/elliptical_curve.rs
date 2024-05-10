use primitive_types::U256;
use rand::{rngs::OsRng, RngCore};

use crate::mod_math::ModMath;

#[derive(Clone, Copy, Debug)]
pub struct ECPoint {
    pub x: U256,
    pub y: U256,
}

impl ECPoint {
    pub fn new(x: U256, y: U256) -> Self {
        Self { x, y }
    }

    pub fn eq(&self, p: &ECPoint) -> bool {
        self.x == p.x && self.y == p.y
    }
}

// Weierstrass Curve 
pub struct Curve {
  // y^2 = x^3 + ax + b mod(p)
  a: U256,
  b: U256,
  pub field_modulus: U256,
  pub curve_order: U256,
  pub G: ECPoint // Generator Point
}

impl Curve {

  pub fn new(a: U256, b: U256, field_modulus: U256, curve_order: U256, G: ECPoint) -> Self {
    Self {
      a,
      b,
      field_modulus,
      curve_order,
      G
    }
  }

  pub fn add_points(&self, p1: &ECPoint, p2: &ECPoint) -> ECPoint {
    if p1.eq(p2) {
      self.point_doubling(p1)
    } else {
      self.point_addition(p1, p2)
    }
  }

  pub fn point_addition(&self, p1: &ECPoint, p2: &ECPoint) -> ECPoint {
      let mod_math = ModMath::new(self.field_modulus);
      let numerator = mod_math.sub(p2.y, p1.y);
      let denominator = mod_math.sub(p2.x, p1.x);
      let slope = mod_math.div(numerator, denominator);
      let slope_squared = mod_math.square(slope);
      let x_3_temp = mod_math.sub(slope_squared, p1.x);
      let x_3 = mod_math.sub(x_3_temp, p2.x);

      let x_diff = mod_math.sub(p1.x, x_3);
      let y_3_temp = mod_math.mul(slope, x_diff);
      let y_3 = mod_math.sub(y_3_temp, p1.y);

      ECPoint {
        x: x_3,
        y: y_3
      }
  }

  pub fn point_doubling(&self, p: &ECPoint) -> ECPoint {
      let mod_math = ModMath::new(self.field_modulus);

      let x_squared = mod_math.square(p.x);
      let three_x_squared = mod_math.mul(x_squared, U256::from(3));
      let numerator = mod_math.add(three_x_squared, self.a);
      let denominator = mod_math.mul(U256::from(2), p.y);
      let slope = mod_math.div(numerator, denominator);

      let slope_squared = mod_math.square(slope);
      let two_p_x = mod_math.mul(U256::from(2), p.x);
      let x_3 = mod_math.sub(slope_squared, two_p_x);

      let p_x_minus_x_3 = mod_math.sub(p.x, x_3);
      let slope_times_x_diff = mod_math.mul(slope, p_x_minus_x_3);
      let y_3 = mod_math.sub(slope_times_x_diff, p.y);

      ECPoint {
        x: x_3,
        y: y_3
      }
  }

  pub fn scalar_multiply_generator(&self, scalar: U256) -> ECPoint {
    self.point_multiplication_scalar(scalar, self.G)
  }

  pub fn point_multiplication_scalar(&self, scalar: U256, starting_point: ECPoint) -> ECPoint {
    let mut r = ECPoint {x: U256::zero(), y: U256::zero()};
    let mut a = starting_point.clone();
    let mut current_scalar = scalar;

    while current_scalar > U256::zero() {
      
      if current_scalar % 2 == U256::one() {
        r = self.point_addition(&r, &a);
      }
      a = self.point_doubling(&a);
      current_scalar = current_scalar / U256::from(2);
    }

    r
  }
}