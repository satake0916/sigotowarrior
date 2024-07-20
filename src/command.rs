use crate::{
    config::MyConfig,
    display::SigoDisplay,
    error::*,
    file::{add_task, read_tasks, ActiveFilable},
    task::{ReadyTask, Task, WaitingTask},
    AppArg, Command,
};

// TODO: DRY get id and match pattern
pub fn run(cfg: &MyConfig, args: AppArg) -> Result<SigoDisplay> {
    match args.command {
        Command::Add {
            description,
            priority,
            waiting,
            due,
        } => {
            let new_task =
                add_task::<ReadyTask>(cfg, ReadyTask::new(cfg, &description, priority, due)?)?;
            if waiting {
                let new_task = new_task.wait(cfg, &None)?;
                Ok(SigoDisplay::CreateWaitingTask(new_task.active_params.id))
            } else {
                Ok(SigoDisplay::CreateReadyTask(new_task.active_params.id))
            }
        }
        Command::Modify { id, priority, due } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    task.modify(cfg, priority, due)?;
                    Ok(SigoDisplay::ModifyTask(
                        task.active_params.id,
                        task.active_params.get_primary_description(),
                    ))
                }
                Task::Waiting(task) => {
                    task.modify(cfg, priority, due)?;
                    Ok(SigoDisplay::ModifyTask(
                        task.active_params.id,
                        task.active_params.get_primary_description(),
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
                        id,
                        task.active_params.get_primary_description(),
                    ))
                }
                Task::Waiting(task) => {
                    task.complete(cfg)?;
                    Ok(SigoDisplay::CompleteTask(
                        id,
                        task.active_params.get_primary_description(),
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
                    Ok(SigoDisplay::WaitTask(
                        task.active_params.id,
                        task.active_params.get_primary_description(),
                    ))
                }
                Task::Waiting(task) => Ok(SigoDisplay::WaitWaitingTask(
                    task.active_params.id,
                    task.active_params.get_primary_description(),
                )),
                Task::Completed(_) => panic!(),
            }
        }
        Command::Back { id, text } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => Ok(SigoDisplay::BackReadyTask(
                    task.active_params.id,
                    task.active_params.get_primary_description(),
                )),
                Task::Waiting(task) => {
                    let task = task.back(cfg, &text)?;
                    Ok(SigoDisplay::BackTask(
                        task.active_params.id,
                        task.active_params.get_primary_description(),
                    ))
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
                        task.active_params.id,
                        task.active_params.get_primary_description(),
                    ))
                }
                Task::Waiting(task) => {
                    task.annotate(cfg, &text)?;
                    Ok(SigoDisplay::AnnotateTask(
                        task.active_params.id,
                        task.active_params.get_primary_description(),
                    ))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::List => {
            let mut tasks = read_tasks::<ReadyTask>(cfg)?;
            tasks.sort();
            Ok(SigoDisplay::ListReadyTasks(tasks))
        }
        Command::Waiting => {
            let mut tasks = read_tasks::<WaitingTask>(cfg)?;
            tasks.sort();
            Ok(SigoDisplay::ListWaitingTasks(tasks))
        }
    }
}
