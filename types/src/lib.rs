#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Submission {
    pub job_id: u8,
    pub result: u32,
    pub proof: String,
}
