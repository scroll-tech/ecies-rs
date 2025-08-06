use std::hint::black_box;
use openvm_k256::Secp256k1Point;
use ecies::SecretKey;

openvm::entry!(main);
openvm::init!();

fn main() {
    let repetitions = openvm::io::read::<u32>();
    let baseline = openvm::io::read::<bool>();

    let sk = openvm::io::read_vec();
    let sk = SecretKey::from_bytes(&sk).unwrap();
    let ciphertext = openvm::io::read_vec();

    let address: [u8; 20] = sk.decrypt(&ciphertext).try_into().unwrap();

    for _ in 0..repetitions {
        let mut ciphertext = ciphertext.clone();
        if !baseline {
            black_box(sk.decrypt_inplace(black_box(&mut ciphertext)));
        } else {
            black_box(&mut ciphertext);
        }
        black_box(ciphertext);
    }

    let mut out: [u8; 32] = [0u8; 32];
    out[..20].copy_from_slice(&address);
    openvm::io::reveal_bytes32(out);
}
