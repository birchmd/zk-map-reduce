use {
    miden::{
        AdviceInputs, Assembler, DefaultHost, MemAdviceProvider, Program, ProvingOptions,
        StackInputs,
    },
    std::collections::HashMap,
};

const SQUARE_MASM: &str = include_str!("../../masm/square.masm");
const CUBE_MASM: &str = include_str!("../../masm/cube.masm");
const DOUBLE_MASM: &str = include_str!("../../masm/double.masm");
const FORTY_TWO_MASM: &str = include_str!("../../masm/forty_two.masm");
const RANDOM_MASM: &str = include_str!("../../masm/random.masm");

const PROGRAMS: [&str; 5] = [
    SQUARE_MASM,
    CUBE_MASM,
    DOUBLE_MASM,
    FORTY_TWO_MASM,
    RANDOM_MASM,
];

pub const PROGRAM_NAMES: [&str; 5] = ["square", "cube", "double", "42", "random"];

pub fn compile_programs() -> HashMap<&'static str, Program> {
    let assembler = Assembler::default();
    PROGRAM_NAMES
        .into_iter()
        .zip(PROGRAMS)
        .map(|(name, masm)| {
            let program = assembler.compile(masm).expect("masm code should be valid");
            (name, program)
        })
        .collect()
}

pub fn prove(program: &Program, input: u8) -> ProveResult {
    let stack_inputs = StackInputs::new(vec![input.into()]);
    let advice_inputs = AdviceInputs::default().with_stack([rand_u8().into()]);
    let host = DefaultHost::new(MemAdviceProvider::from(advice_inputs));
    let (stack_outputs, proof) =
        miden::prove(program, stack_inputs, host, ProvingOptions::default())
            .expect("proof should work");
    ProveResult {
        output_stack: stack_outputs.stack().iter().map(|x| *x as u32).collect(),
        overflow_addrs: stack_outputs.overflow_addrs().to_vec(),
        proof: proof.to_bytes(),
    }
}

fn rand_u8() -> u8 {
    let mut buf = [0u8; 1];
    getrandom::getrandom(&mut buf).ok();
    buf[0]
}

pub struct ProveResult {
    pub output_stack: Vec<u32>,
    pub overflow_addrs: Vec<u64>,
    pub proof: Vec<u8>,
}

impl ProveResult {
    pub fn encoded(&self) -> String {
        base64::encode(&self.proof)
    }
}
