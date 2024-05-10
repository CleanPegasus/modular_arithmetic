#[cfg(test)]
mod tests {
  use crate::galois_field::GaloisField;
  use primitive_types::U256;

  #[test]
  fn test_new_galois_field() {
    let GF7 = GaloisField::new(7).unwrap();
    dbg!(GF7);
  }

  
}