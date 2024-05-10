#[cfg(test)]
mod tests {
  use crate::curves::bn128;
  use primitive_types::U256;

  #[test]
  fn test_point_addition() {
    let bn128 = bn128();
    let G = bn128.G;

    let double_G = bn128.add_points(&G, &G);

    dbg!(double_G);
  }

  #[test]
  fn test_point_doubling() {
    
  }

  #[test]
  fn test_scalar_multiply_generator() {
    let bn128 = bn128();
    let G = bn128.G;

    let double_G = bn128.scalar_multiply_generator(U256::from(2));

    dbg!(double_G);
  }
}