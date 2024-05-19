use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::active_params::ActiveParams;
use crate::config::MyConfig;
use crate::error::SigoError;
use crate::file::{add_task, delete_by_id, get_by_id, read_tasks, ActiveFilable};
use crate::Priority;

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub enum Task {
    Ready(ReadyTask),
    Waiting(WaitingTask),
    Completed(CompletedTask),
}

#[derive(Tabled, Serialize, Deserialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct ReadyTask {
    #[tabled(inline)]
    pub active_params: ActiveParams,
}

#[derive(Tabled, Serialize, Deserialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub struct WaitingTask {
    #[tabled(inline)]
    pub active_params: ActiveParams,
}

#[derive(Tabled, Serialize, Deserialize, Debug, Clone)]
pub struct CompletedTask {
    pub summary: String,
}

impl Task {
    pub fn get_by_id(cfg: &MyConfig, id: u32) -> Result<Task, SigoError> {
        if let Ok(task) = get_by_id::<ReadyTask>(cfg, id) {
            return Ok(Task::Ready(task));
        }
        if let Ok(task) = get_by_id::<WaitingTask>(cfg, id) {
            return Ok(Task::Waiting(task));
        }
        Err(SigoError::TaskNotFound(id))
    }

    fn issue_task_id(cfg: &MyConfig) -> Result<u32, SigoError> {
        let ready_tasks = read_tasks::<ReadyTask>(cfg)?;
        let waiting_tasks = read_tasks::<WaitingTask>(cfg)?;
        let mut using_ids = HashSet::new();
        for task in ready_tasks.iter() {
            using_ids.insert(task.active_params.id);
        }
        for task in waiting_tasks.iter() {
            using_ids.insert(task.active_params.id);
        }
        let max_id: u32 = (using_ids.len() + 1).try_into().unwrap();
        Ok((1u32..=max_id).find(|x| !using_ids.contains(x)).unwrap())
    }
}

impl ReadyTask {
    pub fn new(
        cfg: &MyConfig,
        description: &str,
        priority: Option<Priority>,
    ) -> Result<Self, SigoError> {
        let id = Task::issue_task_id(cfg)?;
        Ok(Self {
            active_params: ActiveParams {
                id,
                description: vec![description.to_owned()],
                priority,
            },
        })
    }

    fn from_waiting(waiting_task: WaitingTask) -> Self {
        ReadyTask {
            active_params: waiting_task.active_params,
        }
    }

    pub fn wait(self, cfg: &MyConfig, text: &Option<String>) -> Result<WaitingTask, SigoError> {
        delete_by_id::<ReadyTask>(cfg, self.active_params.id)?;
        let task = add_task::<WaitingTask>(cfg, WaitingTask::from_ready(self))?;
        if let Some(text) = text {
            task.annotate(cfg, text)?;
        }
        Ok(task)
    }
}
impl WaitingTask {
    fn from_ready(ready_task: ReadyTask) -> Self {
        Self {
            active_params: ready_task.active_params,
        }
    }

    pub fn back(self, cfg: &MyConfig, text: &Option<String>) -> Result<ReadyTask, SigoError> {
        delete_by_id::<WaitingTask>(cfg, self.active_params.id)?;
        let task = add_task::<ReadyTask>(cfg, ReadyTask::from_waiting(self))?;
        if let Some(text) = text {
            task.annotate(cfg, text)?;
        }
        Ok(task)
    }
}
