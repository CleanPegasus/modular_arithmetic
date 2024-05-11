#[cfg(test)]
mod tests {
  use crate::curves::BN128;
  use primitive_types::U256;

  #[test]
  fn test_point_addition() {
    let bn128 = BN128();
    let G = bn128.G;

    let double_G = bn128.add_points(&G, &G);
    let triple_G = bn128.add_points(&double_G, &G);

    dbg!(triple_G);
  }

  #[test]
  fn test_point_doubling() {
    
  }

  #[test]
  fn test_scalar_multiply_generator() {
    let bn128 = BN128();
    let G = bn128.G;

    let double_G = bn128.scalar_multiply_generator(U256::from(2));

    dbg!(double_G);
  }
}