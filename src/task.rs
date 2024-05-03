use std::{collections::HashSet, io::Write, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::config::MyConfig;

#[derive(Serialize, Deserialize, Debug)]
pub enum Task {
    Ready(ReadyTask),
    Waiting(WaitingTask),
    Completed(CompletedTask),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadyTask {
    pub id: u32,
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

macro_rules! create_read_tasks_function {
    () => {
        pub fn read_tasks(cfg: &MyConfig) -> Result<Vec<Self>, std::io::Error> {
            let mut path = PathBuf::from(&cfg.home);
            path.push(Self::FILE_NAME);
            match std::fs::read_to_string(path) {
                Err(err) => Err(err),
                Ok(tasks) => Ok(serde_json::from_str::<Vec<Self>>(&tasks).unwrap()),
            }
        }
    };
}

macro_rules! create_write_tasks_function {
    () => {
        pub fn write_tasks(cfg: &MyConfig, tasks: Vec<Self>) {
            let mut path = PathBuf::from(&cfg.home);
            path.push(Self::FILE_NAME);
            let tmp_path = path.with_extension(format!("sigo-tmp-{}", std::process::id()));
            let mut file = std::fs::File::create(&tmp_path).unwrap();
            let content = serde_json::to_string(&tasks).unwrap();
            std::io::BufWriter::with_capacity(content.len(), &file)
                .write_all(content.as_bytes())
                .unwrap();
            file.flush().unwrap();
            std::fs::rename(&tmp_path, path).unwrap();
        }
    };
}

macro_rules! create_add_task_function {
    () => {
        pub fn add_task(cfg: &MyConfig, task: Self) {
            let mut tasks = Self::read_tasks(cfg).unwrap();
            tasks.push(task);
            Self::write_tasks(cfg, tasks);
        }
    };
}

macro_rules! create_get_by_id_function {
    () => {
        fn get_by_id(cfg: &MyConfig, id: u32) -> Option<Self> {
            let tasks = Self::read_tasks(cfg).unwrap();
            tasks.into_iter().find(|t| t.id == id)
        }
    };
}

macro_rules! create_delete_by_id_function {
    () => {
        fn delete_by_id(cfg: &MyConfig, id: u32) {
            let tasks = Self::read_tasks(cfg).unwrap();
            let updated_tasks = tasks
                .into_iter()
                .filter(|t| t.id != id)
                .collect::<Vec<Self>>();
            Self::write_tasks(cfg, updated_tasks);
        }
    };
}

impl Task {
    pub fn get_by_id(cfg: &MyConfig, id: u32) -> Option<Task> {
        let task = ReadyTask::get_by_id(cfg, id);
        if task.is_some() {
            return Some(Task::Ready(task.unwrap()));
        }

        let task = WaitingTask::get_by_id(cfg, id);
        if task.is_some() {
            return Some(Task::Waiting(task.unwrap()));
        }

        let task = CompletedTask::get_by_id(cfg, id);
        if task.is_some() {
            return Some(Task::Completed(task.unwrap()));
        }

        None
    }

    // REVIEW: DRY
    pub fn complete(&self, cfg: &MyConfig) {
        let completed_task = match &self {
            Task::Ready(task) => {
                let before_tasks = ReadyTask::read_tasks(cfg).unwrap();
                let after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != task.id)
                    .collect::<Vec<ReadyTask>>();
                ReadyTask::write_tasks(cfg, after_tasks);
                CompletedTask {
                    id: task.id,
                    description: task.description.to_owned()
                }
            },
            Task::Waiting(task) => {
                let before_tasks = ReadyTask::read_tasks(cfg).unwrap();
                let after_tasks = before_tasks
                    .into_iter()
                    .filter(|t| t.id != task.id)
                    .collect::<Vec<ReadyTask>>();
                ReadyTask::write_tasks(cfg, after_tasks);
                CompletedTask {
                    id: task.id,
                    description: task.description.to_owned()
                }
            },
            Task::Completed(task) => {
                // TODO: return Result
                CompletedTask {
                    id: task.id,
                    description: task.description.to_owned()
                }
            }
        };
        let mut completed_tasks = CompletedTask::read_tasks(cfg).unwrap();
        completed_tasks.push(completed_task);
        CompletedTask::write_tasks(cfg, completed_tasks);
    }

    fn issue_task_id(cfg: &MyConfig) -> u32 {
        let ready_tasks = ReadyTask::read_tasks(cfg).unwrap();
        let waiting_tasks = WaitingTask::read_tasks(cfg).unwrap();
        let mut using_ids = HashSet::new();
        for task in ready_tasks.iter() {
            using_ids.insert(task.id);
        }
        for task in waiting_tasks.iter() {
            using_ids.insert(task.id);
        }
        let max_id: u32 = (using_ids.len() + 1).try_into().unwrap();
        (1u32..=max_id).find(|x| !using_ids.contains(x)).unwrap()
    }
}

impl ReadyTask {
    const FILE_NAME: &'static str = "ready_tasks";
    create_read_tasks_function!();
    create_write_tasks_function!();
    create_add_task_function!();
    create_get_by_id_function!();
    create_delete_by_id_function!();

    pub fn new(cfg: &MyConfig, description: &str) -> Self {
        let id = Task::issue_task_id(cfg);
        Self {
            id: id,
            description: description.to_owned(),
        }
    }

    pub fn wait(&self, cfg: &MyConfig) {
        ReadyTask::delete_by_id(cfg, self.id);
        WaitingTask::add_task(cfg, WaitingTask::from_ready(self));
    }
}
impl WaitingTask {
    const FILE_NAME: &'static str = "waiting_tasks";
    create_read_tasks_function!();
    create_write_tasks_function!();
    create_add_task_function!();
    create_get_by_id_function!();
    create_delete_by_id_function!();

    pub fn from_ready(ready_task: &ReadyTask) -> Self {
        Self {
            id: ready_task.id,
            description: ready_task.description.to_owned()
        }
    }
}
impl CompletedTask {
    const FILE_NAME: &'static str = "completed_tasks";
    create_read_tasks_function!();
    create_write_tasks_function!();
    create_get_by_id_function!();
    create_delete_by_id_function!();
}
