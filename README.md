# ecies-rs

Elliptic Curve Integrated Encryption Scheme for secp256k1, written in Rust with **minimal** dependencies.

This is the Rust version of [ecies/py](https://github.com/ecies/py) with a built-in class-like secp256k1 API, 
you may go there for detailed documentation of the mechanism under the hood.

## Install

`cargo add --git https://github.com/scroll-tech/ecies-rs`

## Quick Start

```rust
fn main() {
	let secret_key = ecies::SecretKey::random(&mut rand::rng());
    let public_key = secret_key.public_key();

	let mut ciphertext = public_key.encrypt(b"THIS IS THE TEST");
	let plaintext = secret_key.decrypt_in_place(&mut ciphertext);
    assert_eq!(plaintext, b"THIS IS THE TEST");
}
```

