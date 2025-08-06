mod key;
pub use key::{PublicKey, SecretKey};

#[cfg(target_os = "zkvm")]
mod sha256;
#[cfg(not(target_os = "zkvm"))]
pub(crate) use sha2::Sha256;
#[cfg(target_os = "zkvm")]
pub(crate) use sha256::Sha256;

#[cfg(test)]
mod tests;
