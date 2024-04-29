use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Task {
    Ready(ReadyTask),
    Waiting(WaitingTask),
    Completed(CompletedTask),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyTask {
    id: u32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaitingTask {
    id: u32,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletedTask {
    id: u32,
    description: String,
}

impl Task {
    pub fn new(description: &String) -> ReadyTask {
        let id = 0;
        ReadyTask {
            id: id,
            description: description.clone(),
        }
    }
}
