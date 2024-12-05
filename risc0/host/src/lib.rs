use risc0_zkvm::{
    default_prover, ExecutorEnv, Receipt, Result,
};

use methods::{
    ML_GUEST_ELF, ML_GUEST_ID
};

use ml_core::{ModelInput};

pub struct OutputWithReceipt {
    receipt: Receipt
}

impl OutputWithReceipt {
    pub fn get_commit(&self) -> Result<Vec<f32>> {
        Ok(self.receipt.journal.decode()?)
    }

    pub fn verify(&self) -> Result<Vec<f32>> {
            self.receipt.verify(ML_GUEST_ID)?;
            self.get_commit()
        }
}

pub fn run_model(input: ModelInput) -> Result<OutputWithReceipt> {

    let env = ExecutorEnv::builder().write(&input)?.build()?;

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover.prove(env, ML_GUEST_ELF)?.receipt;

    Ok(OutputWithReceipt { receipt })
}
