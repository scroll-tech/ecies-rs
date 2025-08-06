use crate::*;
use std::collections::HashMap;

const TESTING_MESSAGE: &str = "helloworld";
const TESTING_JSON_MESSAGE: &str = r#"{"code":0,"msg":"ok","data":{"pageNumber":1,"pageSize":10,"total":0,"list":[],"realTotal":0}}{"code":0,"msg":"ok","data":{"pageNumber":1,"pageSize":10,"total":0,"list":[],"realTotal":0}}{"code":0,"msg":"ok","data":{"pageNumber":1,"pageSize":10,"total":0,"list":[],"realTotal":0}}"#;
const TESTING_RECEIVER_PUBKEY_HEX: &str = "0498afe4f150642cd05cc9d2fa36458ce0a58567daeaf5fde7333ba9b403011140a4e28911fcf83ab1f457a30b4959efc4b9306f514a4c3711a16a80e3b47eb58b";
const TESTING_RECEIVER_PRIVKEY_HEX: &str =
    "95d3c5e483e9b1d4f5fc8e79b2deaf51362980de62dbb082a9a4257eef653d7d";
const PYTHON_BACKEND: &str = "https://demo.ecies.org/";
const TESTING_RECEIVER_PRIVKEY: [u8; 32] = [
    51, 37, 145, 156, 66, 168, 189, 189, 176, 19, 177, 30, 148, 104, 25, 140, 155, 42, 248, 190,
    121, 110, 16, 174, 143, 148, 72, 129, 94, 113, 219, 58,
];

#[test]
#[cfg(feature = "rand")]
fn test_encrypt_and_decrypt() {
    let receiver_sk = SecretKey::from_bytes(TESTING_RECEIVER_PRIVKEY).unwrap();
    let receiver_pk = receiver_sk.public_key();

    let mut ciphertext = receiver_pk.encrypt(&mut rand::rng(), TESTING_MESSAGE.as_bytes());
    let decrypted_msg = receiver_sk.decrypt_inplace(&mut ciphertext);
    assert_eq!(decrypted_msg, TESTING_MESSAGE.as_bytes());
}

#[test]
#[cfg(feature = "rand")]
fn test_public_key_decompression() {
    let secret_key = SecretKey::random(&mut rand::rng());
    let public_key = secret_key.public_key();

    PublicKey::from_bytes(&public_key.to_bytes(true)).unwrap();
}

#[test]
fn test_decrypt_against_python_version() {
    let mut params = HashMap::new();
    params.insert("data", TESTING_MESSAGE);
    params.insert("pub", TESTING_RECEIVER_PUBKEY_HEX);

    let response = reqwest::blocking::Client::new()
        .post(PYTHON_BACKEND)
        .form(&params)
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap();

    let mut ciphertext = hex::decode(&response).unwrap();
    let receiver_sk =
        SecretKey::from_bytes(hex::decode(TESTING_RECEIVER_PRIVKEY_HEX).unwrap()).unwrap();
    let decrypted_msg = receiver_sk.decrypt_inplace(&mut ciphertext);
    assert_eq!(decrypted_msg, TESTING_MESSAGE.as_bytes());
}

#[test]
#[cfg(feature = "rand")]
fn test_encrypt_against_python_version() {
    let receiver_pk =
        PublicKey::from_bytes(hex::decode(TESTING_RECEIVER_PUBKEY_HEX).unwrap()).unwrap();
    let ciphertext = receiver_pk.encrypt(&mut rand::rng(), TESTING_MESSAGE.as_bytes());

    let mut params = HashMap::new();
    params.insert("data", hex::encode(&ciphertext));
    params.insert("prv", TESTING_RECEIVER_PRIVKEY_HEX.to_string());

    let response = reqwest::blocking::Client::new()
        .post(PYTHON_BACKEND)
        .form(&params)
        .send()
        .unwrap()
        .error_for_status()
        .unwrap()
        .text()
        .unwrap();

    assert_eq!(response, TESTING_MESSAGE);
}
