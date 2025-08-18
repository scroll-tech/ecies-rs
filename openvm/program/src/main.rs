use std::hint::black_box;
use openvm_k256::Secp256k1Point;
use ecies::SecretKey;

openvm::entry!(main);
openvm::init!();

fn main() {
    let repetitions = openvm::io::read::<u32>();
    let baseline = openvm::io::read::<bool>();

    let sk = openvm::io::read_vec();
    let sk = SecretKey::try_from_bytes(&sk).unwrap();
    let ciphertext = openvm::io::read_vec();

    let address: [u8; 20] = sk.try_decrypt(&ciphertext).unwrap().try_into().unwrap();

    for _ in 0..repetitions {
        if !baseline {
            black_box(sk.try_decrypt(black_box(&ciphertext)).unwrap());
        } else {
            black_box(&ciphertext);
        }
        black_box(&ciphertext);
    }

    let mut out: [u8; 32] = [0u8; 32];
    out[..20].copy_from_slice(&address);
    openvm::io::reveal_bytes32(out);
}
