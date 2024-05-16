use crate::{
    config::MyConfig,
    display::SigoDisplay,
    error::*,
    task::{ReadyTask, Task, WaitingTask},
    utils::tasks_to_string,
    AppArg, Command,
};

// TODO: DRY get id and match pattern
pub fn run(cfg: &MyConfig, args: AppArg) -> Result<SigoDisplay> {
    match args.command {
        Command::Add {
            description,
            priority,
            waiting,
        } => {
            let new_task = ReadyTask::add_task(cfg, ReadyTask::new(cfg, &description, priority)?)?;
            if waiting {
                let new_task = new_task.wait(cfg, &None)?;
                Ok(SigoDisplay::CreateWaitingTask(new_task.id))
            } else {
                Ok(SigoDisplay::CreateReadyTask(new_task.id))
            }
        }
        Command::Modify { id, text, priority } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    task.modify(cfg, &text, priority)?;
                    Ok(SigoDisplay::ModifyTask(
                        task.id,
                        task.get_main_description(),
                    ))
                }
                Task::Waiting(task) => {
                    task.modify(cfg, &text, priority)?;
                    Ok(SigoDisplay::ModifyTask(
                        task.id,
                        task.get_main_description(),
                    ))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::Done { id } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    task.complete(cfg)?;
                    Ok(SigoDisplay::CompleteTask(
                        task.id,
                        task.get_main_description(),
                    ))
                }
                Task::Waiting(task) => {
                    task.complete(cfg)?;
                    Ok(SigoDisplay::CompleteTask(
                        task.id,
                        task.get_main_description(),
                    ))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::Wait { id, text } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    let task = task.wait(cfg, &text)?;
                    Ok(SigoDisplay::WaitTask(task.id, task.get_main_description()))
                }
                Task::Waiting(task) => Ok(SigoDisplay::WaitWaitingTask(
                    task.id,
                    task.get_main_description(),
                )),
                Task::Completed(_) => panic!(),
            }
        }
        Command::Back { id, text } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => Ok(SigoDisplay::BackReadyTask(
                    task.id,
                    task.get_main_description(),
                )),
                Task::Waiting(task) => {
                    let task = task.back(cfg, &text)?;
                    Ok(SigoDisplay::BackTask(task.id, task.get_main_description()))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::Annotate { id, text } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    task.annotate(cfg, &text)?;
                    Ok(SigoDisplay::AnnotateTask(
                        task.id,
                        task.get_main_description(),
                    ))
                }
                Task::Waiting(task) => {
                    task.annotate(cfg, &text)?;
                    Ok(SigoDisplay::AnnotateTask(
                        task.id,
                        task.get_main_description(),
                    ))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::List => {
            let mut tasks = ReadyTask::read_tasks(cfg)?;
            tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
            Ok(SigoDisplay::ListReadyTasks(tasks_to_string(tasks)))
        }
        Command::Waiting => {
            let mut tasks = WaitingTask::read_tasks(cfg)?;
            tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
            Ok(SigoDisplay::ListWaitingTasks(tasks_to_string(tasks)))
        }
    }
}
