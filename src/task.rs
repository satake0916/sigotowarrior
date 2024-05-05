use std::{collections::HashSet, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::config::MyConfig;
use crate::error::SigoError;
use crate::utils;

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub enum Task {
    Ready(ReadyTask),
    Waiting(WaitingTask),
    Completed(CompletedTask),
}

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub struct ReadyTask {
    pub id: u32,
    pub description: String,
}

#[derive(Tabled, Serialize, Deserialize, Debug)]
pub struct WaitingTask {
    pub id: u32,
    pub description: String,
}

#[derive(Tabled, Serialize, Deserialize, Debug, Clone)]
pub struct CompletedTask {
    pub id: u32,
    pub description: String,
}

macro_rules! create_read_tasks_function {
    () => {
        pub fn read_tasks(cfg: &MyConfig) -> Result<Vec<Self>, SigoError> {
            let mut path = PathBuf::from(&cfg.home);
            path.push(Self::FILE_NAME);
            utils::create_file_if_not_exist(&path)?;
            let tasks = std::fs::read_to_string(path.clone())
                .map_err(|e| SigoError::FileReadErr(path.clone(), e))?;
            let tasks = serde_json::from_str::<Vec<Self>>(&tasks)
                .map_err(|e| SigoError::ParseStrToTasksErr(path.clone(), e))?;
            Ok(tasks)
        }
    };
}

macro_rules! create_write_tasks_function {
    () => {
        pub fn write_tasks(cfg: &MyConfig, tasks: Vec<Self>) -> Result<(), SigoError> {
            let mut path = PathBuf::from(&cfg.home);
            path.push(Self::FILE_NAME);
            utils::create_file_if_not_exist(&path)?;
            let tmp_path = path.with_extension(format!("sigo-tmp-{}", std::process::id()));
            let mut file = std::fs::File::create(&tmp_path)
                .map_err(|e| SigoError::FileCreateErr(tmp_path.clone(), e))?;
            let tasks = serde_json::to_string(&tasks)?;
            std::io::BufWriter::with_capacity(tasks.len(), &file)
                .write_all(tasks.as_bytes())
                .map_err(|e| SigoError::FileWriteErr(tmp_path.clone(), e))?;
            file.flush()
                .map_err(|e| SigoError::FileWriteErr(tmp_path.clone(), e))?;
            std::fs::rename(&tmp_path, &path)
                .map_err(|e| SigoError::FileRenameErr(tmp_path.clone(), path.clone(), e))?;
            Ok(())
        }
    };
}

macro_rules! create_add_task_function {
    () => {
        pub fn add_task(cfg: &MyConfig, task: Self) -> Result<(), SigoError> {
            let mut tasks = Self::read_tasks(cfg)?;
            tasks.push(task);
            Self::write_tasks(cfg, tasks)?;
            Ok(())
        }
    };
}

macro_rules! create_get_by_id_function {
    () => {
        fn get_by_id(cfg: &MyConfig, id: u32) -> Result<Self, SigoError> {
            let tasks = Self::read_tasks(cfg)?;
            tasks
                .into_iter()
                .find(|t| t.id == id)
                .ok_or(SigoError::TaskNotFound(id))
        }
    };
}

macro_rules! create_delete_by_id_function {
    () => {
        fn delete_by_id(cfg: &MyConfig, id: u32) -> Result<(), SigoError> {
            let tasks = Self::read_tasks(cfg)?;
            let updated_tasks = tasks
                .into_iter()
                .filter(|t| t.id != id)
                .collect::<Vec<Self>>();
            Self::write_tasks(cfg, updated_tasks)?;
            Ok(())
        }
    };
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

    // REVIEW: DRY
    pub fn complete(&self, cfg: &MyConfig) -> Result<CompletedTask, SigoError> {
        let completed_task = match &self {
            Task::Ready(task) => {
                let before_tasks = ReadyTask::read_tasks(cfg)?;
                let after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != task.id)
                    .collect::<Vec<ReadyTask>>();
                ReadyTask::write_tasks(cfg, after_tasks)?;
                CompletedTask {
                    id: task.id,
                    description: task.description.to_owned(),
                }
            }
            Task::Waiting(task) => {
                let before_tasks = WaitingTask::read_tasks(cfg)?;
                let after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != task.id)
                    .collect::<Vec<WaitingTask>>();
                WaitingTask::write_tasks(cfg, after_tasks)?;
                CompletedTask {
                    id: task.id,
                    description: task.description.to_owned(),
                }
            }
            Task::Completed(_) => {
                panic!("caanot complete completed task");
            }
        };
        let mut completed_tasks = CompletedTask::read_tasks(cfg)?;
        completed_tasks.push(completed_task.clone());
        CompletedTask::write_tasks(cfg, completed_tasks)?;
        Ok(completed_task)
    }

    pub fn annotate(&self, cfg: &MyConfig, annotate: &str) -> Result<(), SigoError> {
        match &self {
            Task::Ready(task) => {
                let before_tasks = ReadyTask::read_tasks(cfg)?;
                let mut after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != task.id)
                    .collect::<Vec<ReadyTask>>();
                let annotated_task = ReadyTask {
                    id: task.id,
                    description: format!("{}\n{}", task.description, annotate),
                };
                after_tasks.push(annotated_task);
                ReadyTask::write_tasks(cfg, after_tasks)?;
            }
            Task::Waiting(task) => {
                let before_tasks = WaitingTask::read_tasks(cfg)?;
                let mut after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != task.id)
                    .collect::<Vec<WaitingTask>>();
                let annotated_task = WaitingTask {
                    id: task.id,
                    description: format!("{}\n{}", task.description, annotate),
                };
                after_tasks.push(annotated_task);
                WaitingTask::write_tasks(cfg, after_tasks)?;
            }
            Task::Completed(_) => {
                panic!("cannot annotate completed task");
            }
        };
        Ok(())
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
    create_read_tasks_function!();
    create_write_tasks_function!();
    create_add_task_function!();
    create_get_by_id_function!();
    create_delete_by_id_function!();

    pub fn new(cfg: &MyConfig, description: &str) -> Result<Self, SigoError> {
        let id = Task::issue_task_id(cfg)?;
        Ok(Self {
            id,
            description: description.to_owned(),
        })
    }

    fn from_waiting(waiting_task: &WaitingTask) -> Self {
        ReadyTask {
            id: waiting_task.id,
            description: waiting_task.description.to_owned(),
        }
    }

    pub fn wait(&self, cfg: &MyConfig) -> Result<(), SigoError> {
        ReadyTask::delete_by_id(cfg, self.id)?;
        WaitingTask::add_task(cfg, WaitingTask::from_ready(self))?;
        Ok(())
    }
}
impl WaitingTask {
    const FILE_NAME: &'static str = "waiting_tasks";
    create_read_tasks_function!();
    create_write_tasks_function!();
    create_add_task_function!();
    create_get_by_id_function!();
    create_delete_by_id_function!();

    fn from_ready(ready_task: &ReadyTask) -> Self {
        Self {
            id: ready_task.id,
            description: ready_task.description.to_owned(),
        }
    }

    pub fn back(&self, cfg: &MyConfig) -> Result<(), SigoError> {
        WaitingTask::delete_by_id(cfg, self.id)?;
        ReadyTask::add_task(cfg, ReadyTask::from_waiting(self))?;
        Ok(())
    }
}
impl CompletedTask {
    const FILE_NAME: &'static str = "completed_tasks";
    create_read_tasks_function!();
    create_write_tasks_function!();
}
