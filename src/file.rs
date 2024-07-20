use std::{io::Write, path::PathBuf};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    config::MyConfig,
    error::{Result, SigoError},
    task::{CompletedTask, ReadyTask, WaitingTask},
    utils, Priority,
};

pub trait Filable: Serialize + for<'a> Deserialize<'a> + Clone {
    fn get_filename() -> String;
}

impl Filable for ReadyTask {
    fn get_filename() -> String {
        "ready_tasks".to_string()
    }
}

impl Filable for WaitingTask {
    fn get_filename() -> String {
        "waiting_tasks".to_string()
    }
}

impl Filable for CompletedTask {
    fn get_filename() -> String {
        "completed_tasks".to_string()
    }
}

pub fn read_tasks<T>(cfg: &MyConfig) -> Result<Vec<T>>
where
    T: Filable,
{
    let mut path = PathBuf::from(&cfg.data);
    path.push(T::get_filename());
    utils::create_file_if_not_exist(&path)?;
    let tasks = std::fs::read_to_string(path.clone())
        .map_err(|e| SigoError::FileReadErr(path.clone(), e))?;
    let tasks = serde_json::from_str::<Vec<T>>(&tasks)
        .map_err(|e| SigoError::ParseStrToTasksErr(path.clone(), e))?;
    Ok(tasks)
}

pub fn write_tasks<T>(cfg: &MyConfig, tasks: Vec<T>) -> Result<()>
where
    T: Filable,
{
    let mut path = PathBuf::from(&cfg.data);
    path.push(T::get_filename());
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

pub fn add_task<T>(cfg: &MyConfig, task: T) -> Result<T>
where
    T: Filable,
{
    let mut tasks = read_tasks::<T>(cfg)?;
    tasks.push(task.clone());
    write_tasks::<T>(cfg, tasks)?;
    Ok(task)
}

pub trait ActiveFilable: Filable {
    fn get_id(&self) -> u32;
    fn get_description(&self) -> Vec<String>;
    fn add_annotation(&self, text: &str) -> Self;
    fn modify_params(&self, priority: Option<Priority>, due: Option<NaiveDate>) -> Self;

    fn complete(&self, cfg: &MyConfig) -> Result<CompletedTask> {
        delete_by_id::<Self>(cfg, self.get_id())?;
        let completed_task = CompletedTask {
            summary: self.get_description().concat(),
        };
        add_task::<CompletedTask>(cfg, completed_task.clone())?;
        Ok(completed_task)
    }

    fn annotate(&self, cfg: &MyConfig, text: &str) -> Result<Self> {
        let before_tasks = read_tasks::<Self>(cfg)?;
        let mut after_tasks = before_tasks
            .into_iter()
            .filter(|t| t.get_id() != self.get_id())
            .collect::<Vec<Self>>();
        let annotated_task = self.add_annotation(text);
        after_tasks.push(annotated_task.clone());
        write_tasks::<Self>(cfg, after_tasks)?;
        Ok(annotated_task)
    }

    fn modify(
        &self,
        cfg: &MyConfig,
        priority: Option<Priority>,
        due: Option<NaiveDate>,
    ) -> Result<Self> {
        let before_tasks = read_tasks::<Self>(cfg)?;
        let mut after_tasks = before_tasks
            .into_iter()
            .filter(|t| t.get_id() != self.get_id())
            .collect::<Vec<Self>>();
        let modified_task = self.modify_params(priority, due);
        after_tasks.push(modified_task.clone());
        write_tasks::<Self>(cfg, after_tasks)?;
        Ok(modified_task)
    }
}

pub fn get_by_id<T>(cfg: &MyConfig, id: u32) -> Result<T>
where
    T: ActiveFilable + for<'a> Deserialize<'a>,
{
    let tasks = read_tasks::<T>(cfg)?;
    tasks
        .into_iter()
        .find(|t| t.get_id() == id)
        .ok_or(SigoError::TaskNotFound(id))
}

pub fn delete_by_id<T>(cfg: &MyConfig, id: u32) -> Result<()>
where
    T: ActiveFilable + Serialize + for<'a> Deserialize<'a>,
{
    let tasks = read_tasks::<T>(cfg)?;
    let updated_tasks = tasks
        .into_iter()
        .filter(|t| t.get_id() != id)
        .collect::<Vec<T>>();
    write_tasks::<T>(cfg, updated_tasks)?;
    Ok(())
}

impl ActiveFilable for ReadyTask {
    fn get_id(&self) -> u32 {
        self.active_params.id
    }

    fn get_description(&self) -> Vec<String> {
        self.active_params.description.clone()
    }

    fn add_annotation(&self, text: &str) -> Self {
        ReadyTask {
            active_params: self.active_params.annotate_description(text),
        }
    }

    fn modify_params(&self, priority: Option<Priority>, due: Option<NaiveDate>) -> Self {
        ReadyTask {
            active_params: self.active_params.modify_priority(priority).modify_due(due),
        }
    }
}

impl ActiveFilable for WaitingTask {
    fn get_id(&self) -> u32 {
        self.active_params.id
    }

    fn get_description(&self) -> Vec<String> {
        self.active_params.description.clone()
    }

    fn add_annotation(&self, text: &str) -> Self {
        WaitingTask {
            active_params: self.active_params.annotate_description(text),
        }
    }

    fn modify_params(&self, priority: Option<Priority>, due: Option<NaiveDate>) -> Self {
        WaitingTask {
            active_params: self.active_params.modify_priority(priority).modify_due(due),
        }
    }
}
