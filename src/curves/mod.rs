mod elliptical_curve;
mod curves;
pub use elliptical_curve::{Curve, ECPoint};
pub use curves::BN128;

mod bn128_test;