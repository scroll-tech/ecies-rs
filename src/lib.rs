#[cfg(target_os = "zkvm")]
mod sha256;
#[cfg(not(target_os = "zkvm"))]
use sha2::Sha256;
#[cfg(target_os = "zkvm")]
use sha256::Sha256;

#[cfg(test)]
mod tests;

#[cfg(target_os = "zkvm")]
use openvm_k256 as k256;

use aes_gcm::{AeadInOut, AesGcm, Key, KeyInit, Tag, aead::consts::U16, aes::Aes256, aead};
use hkdf::Hkdf;
use k256::{
    Secp256k1,
    elliptic_curve::{
        self,
        sec1::{EncodedPoint, FromEncodedPoint, ToEncodedPoint},
    },
};

/// Secp256k1 (K-256) public key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicKey(elliptic_curve::PublicKey<Secp256k1>);

/// Secp256k1 (K-256) secret key.
#[derive(Clone)]
pub struct SecretKey(elliptic_curve::SecretKey<Secp256k1>);

const COORDINATE_SIZE: usize = 32; // Size of a field element in bytes
const UNCOMPRESSED_PUBLIC_KEY_SIZE: usize = 1 + 2 * COORDINATE_SIZE; // 1 byte for prefix, 32 bytes for x, 32 bytes for y

const NONCE_SIZE: usize = 16; // Size of nonce for AES-GCM

const TAG_SIZE: usize = 16;

type Aes256Gcm = AesGcm<Aes256, U16>;
type Nonce = aes_gcm::Nonce<U16>;

#[derive(Debug, thiserror::Error)]
#[error("Invalid key")]
pub struct KeyError;

#[derive(Debug, thiserror::Error)]
pub enum DecryptError {
    #[error("Invalid ciphertext length")]
    InvalidLength,
    #[error("Invalid ephemeral pk")]
    InvalidKey(#[from] KeyError),
    #[error("Invalid nonce")]
    InvalidNonce,
    #[error("Invalid tag")]
    InvalidTag,
    #[error("Decryption failed")]
    Failed(#[from] aead::Error),
}

impl PublicKey {
    pub fn try_from_bytes(input: impl AsRef<[u8]>) -> Result<Self, KeyError> {
        EncodedPoint::<Secp256k1>::from_bytes(input)
            .ok()
            .and_then(|point| {
                elliptic_curve::PublicKey::<Secp256k1>::from_encoded_point(&point).into_option()
            })
            .map(PublicKey)
            .ok_or_else(|| KeyError)
    }

    pub fn to_bytes(&self, compressed: bool) -> Box<[u8]> {
        self.0.to_encoded_point(compressed).to_bytes()
    }

    #[cfg(feature = "rand")]
    pub fn encrypt(self, rng: &mut impl rand_core::CryptoRng, msg: impl AsRef<[u8]>) -> Vec<u8> {
        use aes_gcm::AeadCore;

        let msg = msg.as_ref();

        let length = UNCOMPRESSED_PUBLIC_KEY_SIZE + NONCE_SIZE + TAG_SIZE + msg.len();

        let mut ciphertext = Vec::with_capacity(length);

        let ephemeral_sk = SecretKey::random(rng);
        let ephemeral_pk = ephemeral_sk.public_key();
        ciphertext.extend_from_slice(&ephemeral_pk.0.to_encoded_point(false).as_ref());

        let shared_secret = ephemeral_sk.encapsulate(&self);
        let cipher = Aes256Gcm::new(&shared_secret);
        let nonce = Aes256Gcm::generate_nonce_with_rng(rng);
        ciphertext.extend_from_slice(&nonce);

        ciphertext.resize(UNCOMPRESSED_PUBLIC_KEY_SIZE + NONCE_SIZE + TAG_SIZE, 0);
        ciphertext.extend_from_slice(&msg);
        let tag = cipher
            .encrypt_inout_detached(
                &nonce,
                b"",
                (&mut ciphertext[UNCOMPRESSED_PUBLIC_KEY_SIZE + NONCE_SIZE + TAG_SIZE..]).into(),
            )
            .unwrap();
        ciphertext[UNCOMPRESSED_PUBLIC_KEY_SIZE + NONCE_SIZE
            ..UNCOMPRESSED_PUBLIC_KEY_SIZE + NONCE_SIZE + TAG_SIZE]
            .copy_from_slice(&tag);

        ciphertext
    }

    pub fn decapsulate(&self, secret_key: &SecretKey) -> Key<Aes256Gcm> {
        let tweak = secret_key.0.to_nonzero_scalar();

        let shared_point = elliptic_curve::PublicKey::<Secp256k1>::from_affine(
            elliptic_curve::group::Curve::to_affine(&(self.0.to_projective() * tweak.as_ref())),
        )
            .unwrap();

        get_shared_secret(&self.0, &shared_point)
    }
}

impl SecretKey {
    pub fn try_from_bytes(input: impl AsRef<[u8]>) -> Result<Self, KeyError> {
        elliptic_curve::SecretKey::<Secp256k1>::from_slice(input.as_ref())
            .ok()
            .map(SecretKey)
            .ok_or_else(|| KeyError)
    }

    pub fn to_bytes(&self) -> Box<[u8]> {
        self.0.to_bytes().to_vec().into_boxed_slice()
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.public_key())
    }

    #[cfg(feature = "rand")]
    pub fn random(rng: &mut impl rand_core::CryptoRng) -> Self {
        let mut bytes = elliptic_curve::FieldBytes::<Secp256k1>::default();
        loop {
            rng.fill_bytes(&mut bytes);
            if let Some(scalar) = elliptic_curve::NonZeroScalar::from_repr(bytes).into_option() {
                return SecretKey(elliptic_curve::SecretKey::from(scalar));
            }
        }
    }

    pub fn try_decrypt<'a>(&self, ciphertext: &[u8]) -> Result<Vec<u8>, DecryptError> {
        let (ephemeral_pk, nonce, tag, buffer) = split_ciphertext(ciphertext)?;
        let mut buffer = buffer.to_vec();
        self.decrypt_inner(
            ephemeral_pk,
            &nonce,
            &tag,
            buffer.as_mut_slice(),
        )?;
        Ok(buffer)
    }

    pub fn try_decrypt_fixed<'a, const N: usize>(&self, ciphertext: &[u8]) -> Result<[u8; N], DecryptError> {
        let (ephemeral_pk, nonce, tag, buffer) = split_ciphertext(ciphertext)?;
        let mut buffer: [u8; N] = buffer.try_into().unwrap();
        self.decrypt_inner(
            ephemeral_pk,
            &nonce,
            &tag,
            buffer.as_mut_slice(),
        )?;
        Ok(buffer)
    }

    pub fn try_decrypt_inplace<'a>(&self, ciphertext: &'a mut [u8]) -> Result<&'a mut [u8], DecryptError> {
        let (ephemeral_pk, nonce, tag, buffer) = split_ciphertext_mut(ciphertext)?;
        self.decrypt_inner(
            ephemeral_pk,
            &nonce,
            &tag,
            buffer,
        )?;
        Ok(buffer)
    }

    pub fn encapsulate(&self, peer_pk: &PublicKey) -> Key<Aes256Gcm> {
        let tweak = self.0.to_nonzero_scalar();
        let shared_point = elliptic_curve::PublicKey::<Secp256k1>::from_affine(
            elliptic_curve::group::Curve::to_affine(&(peer_pk.0.to_projective() * tweak.as_ref())),
        )
            .unwrap();

        get_shared_secret(&self.public_key().0, &shared_point)
    }

    #[inline]
    fn decrypt_inner(
        &self,
        ephemeral_pk: PublicKey,
        nonce: &Nonce,
        tag: &Tag,
        buffer: &mut [u8],
    ) -> Result<(), DecryptError> {
        let shared_secret = ephemeral_pk.decapsulate(self);
        let cipher = Aes256Gcm::new(&shared_secret);

        cipher
            .decrypt_inout_detached(nonce, b"", buffer.into(), tag)?;
        Ok(())
    }
}

#[inline]
fn get_shared_secret(
    sender_point: &elliptic_curve::PublicKey<Secp256k1>,
    shared_point: &elliptic_curve::PublicKey<Secp256k1>,
) -> Key<Aes256Gcm> {
    let mut secret = [0u8; 2 * UNCOMPRESSED_PUBLIC_KEY_SIZE];

    secret[..UNCOMPRESSED_PUBLIC_KEY_SIZE]
        .copy_from_slice(sender_point.to_encoded_point(false).as_ref());
    secret[UNCOMPRESSED_PUBLIC_KEY_SIZE..]
        .copy_from_slice(shared_point.to_encoded_point(false).as_ref());

    let h = Hkdf::<Sha256>::new(None, &secret);
    let mut shared_secret = [0u8; 32];
    h.expand(b"", &mut shared_secret).unwrap();

    shared_secret.into()
}

#[inline]
fn split_ciphertext(ciphertext: &[u8]) -> Result<(PublicKey, Nonce, Tag, &[u8]), DecryptError> {
    if ciphertext.len() < UNCOMPRESSED_PUBLIC_KEY_SIZE + NONCE_SIZE + TAG_SIZE {
        return Err(DecryptError::InvalidLength);
    }

    let (ephemeral_pk_bytes, remaining) = ciphertext.split_at(UNCOMPRESSED_PUBLIC_KEY_SIZE);
    let ephemeral_pk =
        PublicKey::try_from_bytes(ephemeral_pk_bytes)?;

    let (nonce, remaining) = remaining.split_at(NONCE_SIZE);
    let nonce = Nonce::try_from(nonce).map_err(|_| DecryptError::InvalidNonce)?;
    let (tag, buffer) = remaining.split_at(TAG_SIZE);
    let tag = Tag::try_from(tag).map_err(|_| DecryptError::InvalidTag)?;
    Ok((ephemeral_pk, nonce, tag, buffer))
}

#[inline]
fn split_ciphertext_mut(ciphertext: &mut [u8]) -> Result<(PublicKey, Nonce, Tag, &mut [u8]), DecryptError> {
    if ciphertext.len() < UNCOMPRESSED_PUBLIC_KEY_SIZE + NONCE_SIZE + TAG_SIZE {
        return Err(DecryptError::InvalidLength);
    }

    let (ephemeral_pk_bytes, remaining) = ciphertext.split_at_mut(UNCOMPRESSED_PUBLIC_KEY_SIZE);
    let ephemeral_pk = PublicKey::try_from_bytes(ephemeral_pk_bytes)?;

    let (nonce, remaining) = remaining.split_at_mut(NONCE_SIZE);
    let nonce = Nonce::try_from(&*nonce).map_err(|_| DecryptError::InvalidNonce)?;
    let (tag, buffer) = remaining.split_at_mut(TAG_SIZE);
    let tag = Tag::try_from(&*tag).map_err(|_| DecryptError::InvalidTag)?;
    Ok((ephemeral_pk, nonce, tag, buffer))
}
