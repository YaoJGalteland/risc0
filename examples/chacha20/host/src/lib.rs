#![doc = include_str!("../../README.md")]

use methods::CHACHA20_ENC_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

// By running this demo, Alice can produce a receipt that proves that she knows
// some plaintext and key, such that chacha20.enc(key,plaintext) == ciphertext.
// The inputs key and plaintext are kept secret.
pub fn prove_encryption(key: &[u8; 32], nonce: &[u8; 12], plaintext: &[u8]) -> Receipt {
    let input = (key, nonce, plaintext.to_vec());

    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    prover.prove(env, CHACHA20_ENC_ELF).unwrap().receipt
}
#[cfg(test)]
mod tests {
    use super::*;
    use chacha20::{
        cipher::{KeyIvInit, StreamCipher},
        ChaCha20,
    };
    use methods::CHACHA20_ENC_ID;
    use rand::Rng;
    use rand_core::OsRng;
    #[test]
    fn test_encryption() {
        let mut rng = OsRng;

        // The following key, nonce and plaintext can be replaced by custom generated values
        // sample a random key
        let key: [u8; 32] = rng.gen();
        // sample a random nonce
        let nonce: [u8; 12] = rng.gen();
        // initialize a plaintext
        let plaintext = b"Hello, ChaCha20!";

        let receipt = prove_encryption(&key, &nonce, &plaintext.to_vec());

        // Verify the receipt and then access the journal.
        receipt.verify(CHACHA20_ENC_ID).unwrap();

        let receipt_ciphertext: Vec<u8> = receipt.journal.decode().expect(
            "Journal output should deserialize into the same types (& order) that it was written",
        );

        // Compute the ciphertext
        let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
        let mut ciphertext = plaintext.to_vec();

        // Apply the keystream (encrypt the plaintext).
        cipher.apply_keystream(&mut ciphertext);

        assert_eq!(
            receipt_ciphertext, ciphertext,
            "We expect the zkVM output to be the same as the real ciphertext"
        )
    }
}
