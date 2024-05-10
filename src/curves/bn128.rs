use primitive_types::U256;

use super::{Curve, ECPoint};

pub fn bn128() -> Curve {
  let a = U256::zero();
  let b = U256::from(3);
  let field_modulus = U256::from_dec_str("21888242871839275222246405745257275088696311157297823662689037894645226208583").unwrap();
  let curve_order = U256::from_dec_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").unwrap();
  let G = ECPoint::new(U256::from(1), U256::from(2));
  
  let bn128 = Curve::new(a, b, field_modulus, curve_order, G);

  bn128
}