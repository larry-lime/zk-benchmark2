use methods::{RISC0_GUEST_ELF, RISC0_GUEST_ID};

use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    let input: u32 = 1000;
    let env = ExecutorEnv::builder()
        .write(&input)
        .unwrap()
        .build()
        .unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, RISC0_GUEST_ELF).unwrap();

    // Extract journal of receipt
    let output: u32 = receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("{}", output);
}
