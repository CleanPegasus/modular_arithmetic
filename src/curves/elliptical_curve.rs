use primitive_types::U256;
use rand::{rngs::OsRng, RngCore};

use crate::mod_math::{ModMath, IntoU256};

/// `ECPoint` represents a point on an elliptic curve.
///
/// This struct provides methods for creating a new point and checking
/// if two points are equal.
///
/// # Examples
///
/// ```
/// let point1 = ECPoint::new(5.into_u256(), 7.into_u256());
/// let point2 = ECPoint::new(5.into_u256(), 7.into_u256());
/// assert!(point1.eq(&point2));
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ECPoint {
    pub x: U256,
    pub y: U256,
}

impl ECPoint {

    /// Creates a new `ECPoint` with the given x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let point = ECPoint::new(5.into(), 7.into());
    /// ```
    pub fn new(x: U256, y: U256) -> Self {
        Self { x, y }
    }

    /// Checks if two `ECPoint`s are equal.
    ///
    /// # Examples
    ///
    /// ```
    /// let point1 = ECPoint::new(5.into(), 7.into());
    /// let point2 = ECPoint::new(5.into(), 7.into());
    /// assert!(point1.eq(&point2));
    /// ```
    pub fn eq(&self, p: &ECPoint) -> bool {
        self.x == p.x && self.y == p.y
    }
}

/// `Curve` represents a Weierstrass elliptic curve of form
/// y^2 = x^3 + ax + b mod(p)
///
/// This struct provides methods for creating a new curve and performing
/// point addition, point doubling, and scalar multiplication on the curve.
///
/// # Examples
///
/// ```
/// let G = ECPoint::new(1.into_u256(), 1.into_u256());
/// let curve = Curve::new(0.into(), 7.into_u256(), 11.into_u256(), 5.into_u256(), G);
/// let point = curve.scalar_multiply_generator(2.into_u256());
/// ```
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

  /// Adds two points on the curve.
  ///
  /// If the points are equal, this method performs point doubling.
  /// Otherwise, it performs point addition.
  ///
  /// # Examples
  ///
  /// ```
  /// let curve = ...; // create a curve
  /// let p1 = ECPoint::new(5.into_u256(), 7.into_u256());
  /// let p2 = ECPoint::new(3.into_u256(), 2.into_u256());
  /// let result = curve.add_points(&p1, &p2);
  /// ```
  pub fn add_points(&self, p1: &ECPoint, p2: &ECPoint) -> ECPoint {
    if p1.eq(p2) {
      self.point_doubling(p1)
    } else {
      self.point_addition(p1, p2)
    }
  }

  /// Performs point addition on the curve.
  ///
  /// # Examples
  ///
  /// ```
  /// let curve = ...; // create a curve
  /// let p1 = ECPoint::new(5.into_u256(), 7.into_u256());
  /// let p2 = ECPoint::new(3.into_u256(), 2.into_u256());
  /// let result = curve.point_addition(&p1, &p2);
  /// ```
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

  /// Performs point doubling on the curve.
  ///
  /// # Examples
  ///
  /// ```
  /// let curve = ...; // create a curve
  /// let p = ECPoint::new(5.into_u256(), 7.into_u256());
  /// let result = curve.point_doubling(&p);
  /// ```
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

  /// Performs scalar multiplication of a point on the curve with the generator
  ///
  /// # Examples
  ///
  /// ```
  /// let curve = ...; // create a curve
  /// let scalar = 2.into_u256();
  /// let starting_point = ECPoint::new(5.into_u256(), 7.into_u256());
  /// let result = curve.point_multiplication_scalar(scalar, starting_point);
  /// ```
  pub fn scalar_multiply_generator(&self, scalar: U256) -> ECPoint {
    self.point_multiplication_scalar(scalar, self.G)
  }

  /// Performs scalar multiplication of a point on the curve.
  ///
  /// # Examples
  ///
  /// ```
  /// let curve = ...; // create a curve
  /// let scalar = 2.into_u256();
  /// let starting_point = ECPoint::new(5.into_u256(), 7.into_u256());
  /// let result = curve.point_multiplication_scalar(scalar, starting_point);
  /// ```
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