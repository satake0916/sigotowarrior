use crate::{
    config::MyConfig,
    error::*,
    task::{ReadyTask, Task, WaitingTask},
    utils::tasks_to_string,
    AppArg, Command,
};

// TODO: DRY get id and match pattern
pub fn run(cfg: &MyConfig, args: AppArg) -> Result<String> {
    match args.command {
        Command::Add {
            description,
            priority,
        } => {
            let new_task = ReadyTask::add_task(cfg, ReadyTask::new(cfg, &description, priority)?)?;
            Ok(format!("Created sigo {}", new_task.id))
        }
        Command::Modify { id, text, priority } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    task.modify(cfg, &text, priority)?;
                    Ok(format!("Modify sigo {}", task.id))
                }
                Task::Waiting(task) => {
                    task.modify(cfg, &text, priority)?;
                    Ok(format!("Modify sigo {}", task.id))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::Done { id } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    task.complete(cfg)?;
                    Ok(format!("Completed sigo {}", task.id))
                }
                Task::Waiting(task) => {
                    task.complete(cfg)?;
                    Ok(format!("Completed sigo {}", task.id))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::Wait { id } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    let task = task.wait(cfg)?;
                    Ok(format!(
                        "Waiting sigo {} '{}'",
                        task.id,
                        task.get_main_description()
                    ))
                }
                Task::Waiting(task) => Ok(format!("Already waiting sigo {}", task.id)),
                Task::Completed(_) => panic!(),
            }
        }
        Command::Back { id } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => Ok(format!(
                    "Already ready sigo {} '{}'",
                    task.id,
                    task.get_main_description()
                )),
                Task::Waiting(task) => {
                    let task = task.back(cfg)?;
                    Ok(format!("Returning sigo {}", task.id))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::Annotate { id, text } => {
            let task = Task::get_by_id(cfg, id)?;
            match task {
                Task::Ready(task) => {
                    task.annotate(cfg, &text)?;
                    Ok(format!("Annotated sigo {}", task.id))
                }
                Task::Waiting(task) => {
                    task.annotate(cfg, &text)?;
                    Ok(format!("Annotated sigo {}", task.id))
                }
                Task::Completed(_) => panic!(),
            }
        }
        Command::List => {
            let mut tasks = ReadyTask::read_tasks(cfg)?;
            tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
            Ok(tasks_to_string(tasks))
        }
        Command::Waiting => {
            let mut tasks = WaitingTask::read_tasks(cfg)?;
            tasks.sort_by(|a, b| a.priority.cmp(&b.priority));
            Ok(tasks_to_string(tasks))
        }
    }
}
