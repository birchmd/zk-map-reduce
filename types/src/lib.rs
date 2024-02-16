#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Submission {
    pub worker_id: String,
    pub job_id: u8,
    pub output_stack: Vec<u32>,
    pub overflow_addrs: Vec<String>,
    pub proof: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq)]
pub struct Response {
    pub result: String,
}

impl Response {
    pub fn new(result: String) -> Self {
        Self { result }
    }
}
