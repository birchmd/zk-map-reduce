use miden::{Assembler, DefaultHost, Program, ProvingOptions, StackInputs};

const PROGRAM: &str = include_str!("../../masm/map.masm");

pub fn compile_program() -> Program {
    let assembler = Assembler::default();
    assembler
        .compile(PROGRAM)
        .expect("masm code should be valid")
}

pub fn prove(program: &Program, input: u8) -> ProveResult {
    let stack_inputs = StackInputs::new(vec![input.into()]);
    let host = DefaultHost::default();
    let (stack_outputs, proof) =
        miden::prove(program, stack_inputs, host, ProvingOptions::default())
            .expect("proof should work");
    let value = stack_outputs.stack()[0] as u32;
    ProveResult {
        value,
        proof: proof.to_bytes(),
    }
}

pub struct ProveResult {
    pub value: u32,
    pub proof: Vec<u8>,
}

impl ProveResult {
    pub fn encoded(&self) -> String {
        base64::encode(&self.proof)
    }
}
