use {
    crate::verifier,
    miden::Program,
    std::{collections::HashSet, sync::Arc},
    tokio::{
        sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
        task::JoinHandle,
    },
};

pub struct State {
    program: Arc<Program>,
    channel: UnboundedReceiver<Msg>,
    pending_jobs: HashSet<u8>,
    job_results: Vec<u32>,
}

impl State {
    pub fn create() -> (Self, UnboundedSender<Msg>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let result = Self {
            channel: rx,
            program: Arc::new(verifier::compile_program()),
            pending_jobs: (1..=10).collect(),
            job_results: Vec::new(),
        };
        (result, tx)
    }

    pub fn program(&self) -> Arc<Program> {
        self.program.clone()
    }

    pub fn spawn_actor(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(msg) = self.channel.recv().await {
                match msg {
                    Msg::CompletedJob {
                        worker_id,
                        job_id: id,
                        result,
                    } => {
                        if self.pending_jobs.contains(&id) {
                            self.pending_jobs.remove(&id);
                            let message =
                                format!("Job ID {id} completed by {worker_id}. Result = {result}");
                            self.job_results.push(result);
                            self.handle_log(message);
                            if self.pending_jobs.is_empty() {
                                let final_result: u32 = self.job_results.iter().copied().sum();
                                let message =
                                    format!("All jobs complete. Final result = {final_result}");
                                self.handle_log(message);
                            }
                        }
                    }
                    Msg::Log { message } => self.handle_log(message),
                }
            }
        })
    }

    fn handle_log(&self, message: String) {
        // TODO: properly handle logs
        println!("{message}")
    }
}

#[derive(Debug, Clone)]
pub enum Msg {
    CompletedJob {
        worker_id: String,
        job_id: u8,
        result: u32,
    },
    Log {
        message: String,
    },
}
