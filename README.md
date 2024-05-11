# Modular Arithmetic Library

![crates.io](https://img.shields.io/crates/v/modular_math.svg)  [![Docs.rs](https://docs.rs/crate_name/badge.svg)](https://docs.rs/modular_math/0.1.6/)


`modular_math` is a Rust library designed for high-performance modular arithmetic operations on 256-bit integers (`U256`). This library provides robust functionalities such as
- [Modular Arithmetic](#modmath)
  - addition
  - subtraction
  - multiplication
  - exponentiation 
  - inverse
  - divide
  - square
  - square root (tonelli shanks algorithm)
  - equivalent (congruent)
- [Elliptical Curves](#elliptic-curves)
  - Point addition
  - Point doubling
  - Scalar multiplication
  - Scalar multiplication with Generator Point
  - BN128 Curve
  - Secp256k1 Curve
- Galois Fields (Work in Progress)
  - Polynomial
  
under a specified modulus, specifically optimized for cryptographic and zero knowledge applications where such operations are frequently required.

## Features

- **Comprehensive Modular Arithmetic and Elliptical Curves**: Offers all the necessary modular arithmetic operations on `U256` and BN128 and Secp256k1 elliptic curves.
- **Safe Overflow Management**: Utilizes `U512` for intermediate results to prevent overflows.
- **Flexible Type Support**: Features `IntoU256` trait to convert various integer and string types to `U256`.
- **High Performance**: Optimized for performance without compromising on accuracy, especially suitable for cryptographic and zero knowledge applications.
- **Ease of Use Macros** : Provides macros for easy usage of number under a modulus.

## Structure
The workspace is organized as follows:

- src/curves/: Contains the implementation of elliptic curves and points on the curve, BN128 and Secp256k1 Elliptic Curves.
- src/galois_field/: Contains the implementation of Galois fields.
- src/mod_math/: Contains modular arithmetic functions.
- src/num_mod/: Contains the implementation of a number modulo some modulus.

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
modular_math = "0.1.6"
```
##### ModMath

```rust
use modular_math::ModMath;
use primitive_types::U256;

let modulus = "101";
let mod_math = ModMath::new(modulus);

// Addition
let sum = mod_math.add(8, 12);
assert_eq!(sum, U256::from(3));

// Subtraction
let sub = mod_math.sub(8, 12);
assert_eq!(sub, U256::from(97));

// Multiplication
let mul = mod_math.mul(8, 12);
assert_eq!(mul, U256::from(96));

// Multiplicative Inverse
let inv = mod_math.inv(8);
assert_eq!(inv, U256::from(77));

// Exponentiation
let exp = mod_math.exp(8, 12);
assert_eq!(exp, U256::from(64));

// FromStr
let mod_math = ModMath::new("115792089237316195423570985008687907852837564279074904382605163141518161494337");
let div = mod_math.div("32670510020758816978083085130507043184471273380659243275938904335757337482424" , "55066263022277343669578718895168534326250603453777594175500187360389116729240");
assert_eq!(div, U256::from_dec_str("13499648161236477938760301359943791721062504425530739546045302818736391397630"));
```

##### Elliptic Curves

**Create an Elliptic Curve**
```rust
use primitive_types::U256;
use modular_math::elliptical_curve::{Curve, ECPoint};

fn BN128() -> Curve {
  let a = U256::zero();
  let b = U256::from(3);
  let field_modulus = U256::from_dec_str("21888242871839275222246405745257275088696311157297823662689037894645226208583").unwrap();
  let curve_order = U256::from_dec_str("21888242871839275222246405745257275088548364400416034343698204186575808495617").unwrap();
  let G = ECPoint::new(U256::from(1), U256::from(2));
  
  let bn128 = Curve::new(a, b, field_modulus, curve_order, G);

  bn128
}

```

**BN128 Usage**
```rust
use modular_math::curves::BN128;
use primitive_types::U256;

let bn128 = BN128();
let G = bn128.G;

// Scalar Multiplication
let double_G = bn128.point_multiplication_scalar(2, &G);

// Point Addition
let triple_G = bn128.point_addition(&double_G, &G);

// Point Doubling
let quad_G = bn128.point_doubling(&double_G);

```

##### Number Under Modulus
```rust
use modular_math::number_mod::NumberUnderMod as NM;
use primitive_types::U256;
use modular_math::num_mod;

// Add
let a = num_mod!(10, 13);
let b = num_mod!(6, 13);
let sum = a + b;
assert_eq!(sum, U256::from(3));

// Sub
let sub = a - b;
assert_eq!(sub, U256::from(4));

// Mul
let mul = a * b;
assert_eq!(mul, U256::from(8));

// Div
let div = a / b;
assert_eq!(div, U256::from(11));

// Neg
let neg = -a;
assert_eq!(neg, U256::from(3));

let c = num_mod!(10, 13);
assert!(a == c);
```

## Todo
- [x] Square root under modulus
- [x] Additive Inverse
- [ ] BigNumber Tests
- [x] Elliptic Curves
- [ ] Galois Field
- [ ] Bilinear Pairing

## License
This project is licensed under the MIT License - see the LICENSE file for details.



