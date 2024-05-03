# Modular Arithmetic Library

This is a simple modular arithmetic library for Rust, operating on unsigned 256-bit integers (`U256`). It provides a `ModMath` struct that performs operations under a given modulus.

## Features

- Addition, subtraction, multiplication, and division under a modulus
- Exponentiation under a modulus
- Calculation of modular multiplicative inverse
- Equality check under a modulus

## Usage

First, add this to your `Cargo.toml`:

```toml
[dependencies]
modular_math = "0.1.0"
```
Then, you can use the ModMath struct in your code:

```rust
use modular_math::ModMath;
use primitive_types::U256;

let modulus = U256::from(17);
let mod_math = ModMath::new(modulus);
let result = mod_math.add(U256::from(8), U256::from(12));
assert_eq!(result, U256::from(3));
```

## License
This project is licensed under the MIT License - see the LICENSE file for details.



