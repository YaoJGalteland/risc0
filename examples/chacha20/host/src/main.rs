use host::prove_encryption;
use methods::CHACHA20_ENC_ID;
use rand::Rng;
use rand_core::OsRng;
fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let mut rng = OsRng;

    // The following key, nonce and plaintext can be replaced by custom generated values
    // sample a random key
    let key: [u8; 32] = rng.gen();
    // sample a random nonce
    let nonce: [u8; 12] = rng.gen();
    // initialize a plaintext
    let plaintext = b"Hello, ChaCha20!";

    // Run encryption verified in the zkVM guest and get the resulting receipt.
    let receipt = prove_encryption(&key, &nonce, &plaintext.to_vec());

    // Verify the receipt and then access the journal.
    receipt.verify(CHACHA20_ENC_ID).unwrap();

    let receipt_ciphertext: Vec<u8> = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    println!(
        "I know the plaintext and the key of the ciphertext {:?}, and I can prove it!",
        receipt_ciphertext
    );
}
