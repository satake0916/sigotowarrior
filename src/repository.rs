use std::path::PathBuf;

use crate::task::{ReadyTask, Task};

pub fn insert_task(task: Task, path: PathBuf){
    println!("insert_task");
    read_tasks(path).unwrap();
}

pub fn read_tasks(path: PathBuf) -> Result<Option<()>, std::io::Error>{
    match std::fs::read_to_string(&path){
        Err(err) => Err(err),
        Ok(tasks) => {
            let desirialized: Vec<ReadyTask>= serde_json::from_str(&tasks).unwrap();

            for data in &desirialized {
                println!("{:?}", data)
            }
            Ok(Some(()))
        }
   }
}