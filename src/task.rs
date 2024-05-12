use std::{collections::HashSet, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::config::MyConfig;
use crate::error::SigoError;
use crate::{utils, Priority};

use sigo_macro_derive::{FiledTask, IdAssignedTask};

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub enum Task {
    Ready(ReadyTask),
    Waiting(WaitingTask),
    Completed(CompletedTask),
}

#[derive(Tabled, Serialize, Deserialize, Debug, Clone, FiledTask, IdAssignedTask)]
pub struct ReadyTask {
    pub id: u32,
    #[tabled(rename = "P")]
    pub priority: Priority,
    #[tabled(display_with = "utils::display_option_vec_string")]
    pub description: Option<Vec<String>>,
}

#[derive(Tabled, Serialize, Deserialize, Debug, Clone, FiledTask, IdAssignedTask)]
pub struct WaitingTask {
    pub id: u32,
    #[tabled(rename = "P")]
    pub priority: Priority,
    #[tabled(display_with = "utils::display_option_vec_string")]
    pub description: Option<Vec<String>>,
}

#[derive(Tabled, Serialize, Deserialize, Debug, Clone, FiledTask)]
pub struct CompletedTask {
    pub description: String,
}

impl Task {
    pub fn get_by_id(cfg: &MyConfig, id: u32) -> Result<Task, SigoError> {
        if let Ok(task) = ReadyTask::get_by_id(cfg, id) {
            return Ok(Task::Ready(task));
        }
        if let Ok(task) = WaitingTask::get_by_id(cfg, id) {
            return Ok(Task::Waiting(task));
        }
        Err(SigoError::TaskNotFound(id))
    }

    fn issue_task_id(cfg: &MyConfig) -> Result<u32, SigoError> {
        let ready_tasks = ReadyTask::read_tasks(cfg)?;
        let waiting_tasks = WaitingTask::read_tasks(cfg)?;
        let mut using_ids = HashSet::new();
        for task in ready_tasks.iter() {
            using_ids.insert(task.id);
        }
        for task in waiting_tasks.iter() {
            using_ids.insert(task.id);
        }
        let max_id: u32 = (using_ids.len() + 1).try_into().unwrap();
        Ok((1u32..=max_id).find(|x| !using_ids.contains(x)).unwrap())
    }
}

impl ReadyTask {
    const FILE_NAME: &'static str = "ready_tasks";

    pub fn new(cfg: &MyConfig, description: &str, priority: Priority) -> Result<Self, SigoError> {
        let id = Task::issue_task_id(cfg)?;
        Ok(Self {
            id,
            description: Some(vec![description.to_owned()]),
            priority,
        })
    }

    fn from_waiting(waiting_task: WaitingTask) -> Self {
        ReadyTask {
            id: waiting_task.id,
            description: Some(waiting_task.description.unwrap_or_default()),
            priority: waiting_task.priority,
        }
    }

    pub fn wait(self, cfg: &MyConfig, text: &Option<String>) -> Result<WaitingTask, SigoError> {
        ReadyTask::delete_by_id(cfg, self.id)?;
        let task = WaitingTask::add_task(cfg, WaitingTask::from_ready(self))?;
        if let Some(text) = text {
            task.annotate(cfg, text)?;
        }
        Ok(task)
    }
}
impl WaitingTask {
    const FILE_NAME: &'static str = "waiting_tasks";

    fn from_ready(ready_task: ReadyTask) -> Self {
        Self {
            id: ready_task.id,
            description: Some(ready_task.description.unwrap_or_default()),
            priority: ready_task.priority,
        }
    }

    pub fn back(self, cfg: &MyConfig, text: &Option<String>) -> Result<ReadyTask, SigoError> {
        WaitingTask::delete_by_id(cfg, self.id)?;
        let task = ReadyTask::add_task(cfg, ReadyTask::from_waiting(self))?;
        if let Some(text) = text {
            task.annotate(cfg, text)?;
        }
        Ok(task)
    }
}
impl CompletedTask {
    const FILE_NAME: &'static str = "completed_tasks";
}
