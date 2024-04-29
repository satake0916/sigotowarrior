use serde::{Deserialize, Serialize};
use std::{fs::read, io::Write, path::PathBuf};

use crate::config::MyConfig;
use crate::task::{ReadyTask, Task};

// TODO return Result
pub fn insert_task(cfg: &MyConfig, task: ReadyTask) {
    let result = read_ready_tasks(cfg);
    // TODO deal with errors
    let mut tasks = result.unwrap();
    tasks.push(task);
    write_ready_tasks(&cfg, tasks)
}

pub fn read_ready_tasks(cfg: &MyConfig) -> Result<Vec<ReadyTask>, std::io::Error> {
    let mut path = PathBuf::from(&cfg.home);
    path.push("ready_tasks");
    // return Ok(serde_json::from_str::<Vec<ReadyTask>>(&(std::fs::read_to_string(path)?)).unwrap());
    match std::fs::read_to_string(path) {
        Err(err) => Err(err),
        Ok(tasks) => Ok(serde_json::from_str::<Vec<ReadyTask>>(&tasks).unwrap()),
    }
}

pub fn write_ready_tasks(cfg: &MyConfig, tasks: Vec<ReadyTask>) {
    let mut path = PathBuf::from(&cfg.home);
    path.push("ready_tasks");
    let tmp_path = path.with_extension(format!("sigo-tmp-{}", std::process::id()));
    let mut file = std::fs::File::create(&tmp_path).unwrap();
    let content = serde_json::to_string(&tasks).unwrap();
    std::io::BufWriter::with_capacity(content.len(), &file)
        .write_all(content.as_bytes())
        .unwrap();
    file.flush().unwrap();
    std::fs::rename(&tmp_path, path).unwrap();
}
