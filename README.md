# Modular Arithmetic Library

`ModMath` is a Rust library designed for high-performance modular arithmetic operations on 256-bit integers (`U256`). This library provides robust arithmetic functionalities such as 
- addition
- subtraction
- multiplication
- exponentiation 
- inverse
- divide
- square
- equivalent (congruent)
under a specified modulus, specifically optimized for cryptographic and zero knowledge applications where such operations are frequently required.

## Features

- **Comprehensive Modular Arithmetic**: Offers modular addition, subtraction, multiplication, inverse, division, and exponentiation.
- **Safe Overflow Management**: Utilizes `U512` for intermediate results to prevent overflows.
- **Flexible Type Support**: Features `IntoU256` trait to convert various integer and string types to `U256`.
- **High Performance**: Optimized for performance without compromising on accuracy, especially suitable for cryptographic and zero knowledge applications.

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
modular_math = "0.1.5"
```
Then, you can use the ModMath struct in your code:

```rust
use modular_math::ModMath;
use primitive_types::U256;

let modulus = "101";
let mod_math = ModMath::new(modulus);
```

**FromStr**
```rust
let mod_math = ModMath::new("115792089237316195423570985008687907852837564279074904382605163141518161494337");
let div = mod_math.div("32670510020758816978083085130507043184471273380659243275938904335757337482424" , "55066263022277343669578718895168534326250603453777594175500187360389116729240");
assert_eq!(div, U256::from_dec_str("13499648161236477938760301359943791721062504425530739546045302818736391397630"));
```

**Addition**
```rust
let sum = mod_math.add(8, 12);
assert_eq!(sum, U256::from(3));
```
**Subtraction**
```rust
let sub = mod_math.sub(8, 12);
assert_eq!(sub, U256::from(97));
```
**Multiplication**
```rust
let mul = mod_math.mul(8, 12);
assert_eq!(mul, U256::from(96));
```
**Inverse**
```rust
let inv = mod_math.inv(8);
assert_eq!(inv, U256::from(77));
```
**Exponentiation**
```rust
let exp = mod_math.exp(8, 12);
assert_eq!(exp, U256::from(64));
```

## License
This project is licensed under the MIT License - see the LICENSE file for details.



