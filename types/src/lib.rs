#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Submission {
    pub worker_id: String,
    pub job_id: u8,
    pub result: u32,
    pub overflow_addrs: Vec<String>,
    pub proof: String,
}
