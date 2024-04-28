pub struct ReadyTask{
    id: u32,
    description: String
}
pub struct WaitingTask{
    id: u32,
    description: String
}
pub struct CompletedTask{
    id: u32,
    description: String
}
pub enum Task {
    Ready(ReadyTask),
    Waiting(WaitingTask),
    Completed(CompletedTask),
}

impl Task {
    pub fn new(
        description: &String
    ) -> ReadyTask {
        let id = 0;
        ReadyTask {
            id: id,
            description: description.clone()
        }
    }
}