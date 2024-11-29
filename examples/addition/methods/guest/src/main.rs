
#![no_main]
#![no_std]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn main() {
    // Load the first number from the host
    let a: u64 = env::read();
    // Load the second number from the host
    let b: u64 = env::read();

    // Compute the addition while being careful with integer overflow
    let product = a.checked_add(b).expect("Integer overflow");
    env::commit(&product);
}