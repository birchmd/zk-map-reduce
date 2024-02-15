use miden::{Assembler, ExecutionProof, Program, ProgramInfo, StackInputs, StackOutputs};
use zkmr_types::Submission;

const PROGRAM: &str = include_str!("../../masm/map.masm");

pub fn compile_program() -> Program {
    let assembler = Assembler::default();
    assembler
        .compile(PROGRAM)
        .expect("masm code should be valid")
}

pub fn validate_submission(program: &Program, submission: &Submission) -> Result<u32, SubmitError> {
    let stack_inputs = StackInputs::new(vec![submission.job_id.into()]);
    let overflow_addrs: Result<Vec<u64>, SubmitError> = submission
        .overflow_addrs
        .iter()
        .map(|x| x.parse().map_err(|_| SubmitError::InvalidOverflow))
        .collect();
    let stack_outputs = StackOutputs::new(vec![submission.result.into()], overflow_addrs?)
        .map_err(|_| SubmitError::InvalidOutputs)?;
    let proof_bytes = base64::decode(&submission.proof).map_err(|_| SubmitError::InvalidBase64)?;
    let proof = ExecutionProof::from_bytes(&proof_bytes)
        .map_err(|_| SubmitError::ProofDeserializeFailure)?;
    let program_info = ProgramInfo::new(program.hash(), program.kernel().clone());
    let verify_result = miden::verify(program_info, stack_inputs, stack_outputs, proof)
        .map_err(|_| SubmitError::VerifyFailure)?;
    Ok(verify_result)
}

#[derive(Debug)]
pub enum SubmitError {
    InvalidOverflow,
    InvalidOutputs,
    InvalidBase64,
    ProofDeserializeFailure,
    VerifyFailure,
}
