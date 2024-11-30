#![doc = include_str!("../../README.md")]

use methods::ADD_ELF;
use risc0_zkvm::{default_prover, ExecutorEnv, Receipt};

// By running this demo, Alice can produce a receipt that proves that she knows
// some numbers a and b, such that a+b == 110.
// The inputs a and b are kept secret.

// Compute the addition a+b inside the zkVM
pub fn add(a: u64, b: u64) -> (Receipt, u64) {
    // Step 2 (Host): Environment Setup  and Share Private Data as Input with the Guest
    let env = ExecutorEnv::builder()
        // Send a & b to the guest
        .write(&a)
        .unwrap()
        .write(&b)
        .unwrap()
        .build()
        .unwrap();

    // Step 4 (Host): Proof Generation and Validation
    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, ADD_ELF).unwrap().receipt;

    // Extract journal of receipt (i.e. output c, where c = a + b)
    let c: u64 = receipt.journal.decode().expect(
        "Journal output should deserialize into the same types (& order) that it was written",
    );

    // Report the addition
    println!(
        "I know the inputs of the addition {}, and I can prove it!",
        c
    );

    (receipt, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        const TEST_FACTOR_ONE: u64 = 17;
        const TEST_FACTOR_TWO: u64 = 23;
        let (_, result) = add(17, 23);
        assert_eq!(
            result,
            TEST_FACTOR_ONE + TEST_FACTOR_TWO,
            "We expect the zkVM output to be the addition of the inputs"
        )
    }
}
