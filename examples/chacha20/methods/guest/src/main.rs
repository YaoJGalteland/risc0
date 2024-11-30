#![no_main]

use chacha20::{
    cipher::{KeyIvInit, StreamCipher},
    ChaCha20,
};
use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

fn main() {
    // Step 3 (Guest): Read Input and Commit Output
    // Decode the key, nonce, and plaintext from the inputs.
    let (key, nonce, plaintext): ([u8; 32], [u8; 12], Vec<u8>) = env::read();

    // Create a ChaCha20 cipher instance with the given key and nonce.
    let mut cipher = ChaCha20::new(&key.into(), &nonce.into());
    let mut ciphertext = plaintext.to_vec();

    // Apply the keystream (encrypt the plaintext).
    cipher.apply_keystream(&mut ciphertext);

    // Commit the ciphertext to the journal.
    env::commit(&ciphertext);
}
