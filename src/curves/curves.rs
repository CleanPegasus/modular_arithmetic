use primitive_types::U256;

use super::{Curve, ECPoint};

pub fn BN128() -> Curve {
  let a = U256::zero();
  let b = U256::from(3);
  let field_modulus = U256::from_dec_str("21888242871839275222246405745257275088696311157297823662689037894645226208583").unwrap();
  let curve_order = U256::from_dec_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").unwrap();
  let G = ECPoint::new(U256::from(1), U256::from(2));
  
  let bn128 = Curve::new(a, b, field_modulus, curve_order, G);

  bn128
}

pub fn Secp256k1() -> Curve {
  let a: U256 = U256::from(0_u32);
  let b: U256 = U256::from(7_u32);
    
  let field_modulus: U256 = U256::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap(); // p = 2^256 −2^32 −977

  let G = ECPoint::new(U256::from_dec_str("55066263022277343669578718895168534326250603453777594175500187360389116729240").unwrap(), 
                    U256::from_dec_str("32670510020758816978083085130507043184471273380659243275938904335757337482424").unwrap());

  let curve_order = U256::from_str_radix("0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();
   
  let secp256k1 = Curve::new(a, b, field_modulus, curve_order, G);

  secp256k1
}