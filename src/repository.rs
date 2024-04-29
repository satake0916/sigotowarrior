use serde::{Deserialize, Serialize};
use std::{fs::read, io::Write, path::PathBuf};

use crate::task::{ReadyTask, Task};

pub fn insert_task(task: ReadyTask, path: PathBuf) {
    let result = read_ready_tasks(&path);
    let mut tasks = result.unwrap();
    tasks.push(task);

    write_ready_tasks(&path, tasks)
}

pub fn read_ready_tasks(path: &PathBuf) -> Result<Vec<ReadyTask>, std::io::Error> {
    match std::fs::read_to_string(path) {
        Err(err) => Err(err),
        Ok(tasks) => {
            Ok(serde_json::from_str::<Vec<ReadyTask>>(&tasks).unwrap())
        }
    }
}

pub fn write_ready_tasks(path: &PathBuf, tasks: Vec<ReadyTask>) {
    let tmp_path = path.with_extension(format!("sigo-tmp-{}", std::process::id()));
    let mut file = std::fs::File::create(&tmp_path).unwrap();
    let content = serde_json::to_string(&tasks).unwrap();
    std::io::BufWriter::with_capacity(content.len(), &file)
        .write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
    std::fs::rename(&tmp_path, path);
}