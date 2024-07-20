pub enum SigoDisplay {
    CreateReadyTask(u32),
    CreateWaitingTask(u32),
    ModifyTask(u32, String),
    CompleteTask(u32, String),
    WaitTask(u32, String),
    WaitWaitingTask(u32, String),
    BackTask(u32, String),
    BackReadyTask(u32, String),
    AnnotateTask(u32, String),
    ListReadyTasks(Vec<ReadyTask>),
    ListWaitingTasks(Vec<WaitingTask>),
}

use std::fmt::{self, Display};

use crate::{
    config::{Mode, MyConfig},
    task::{ReadyTask, WaitingTask},
    utils::tasks_to_string,
};

impl SigoDisplay {
    pub fn display<'a>(&'a self, cfg: &MyConfig) -> Box<dyn Display + 'a> {
        match cfg.mode {
            Mode::Minimum => Box::new(DisplayMinimum(self)),
            Mode::Simple => Box::new(DisplaySimple(self)),
        }
    }
}

pub struct DisplayMinimum<'a>(&'a SigoDisplay);
impl fmt::Display for DisplayMinimum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            SigoDisplay::CreateReadyTask(id) => {
                writeln!(f, "Created sigo {}", id)
            }
            SigoDisplay::CreateWaitingTask(id) => {
                writeln!(f, "Created waiting sigo {}", id)
            }
            SigoDisplay::ModifyTask(id, description) => {
                writeln!(f, "Modify sigo {} '{}'", id, description)
            }
            SigoDisplay::CompleteTask(id, description) => {
                writeln!(f, "Complete sigo {} '{}'", id, description)
            }
            SigoDisplay::WaitTask(id, description) => {
                writeln!(f, "Waiting sigo {} '{}'", id, description)
            }
            SigoDisplay::WaitWaitingTask(id, description) => {
                writeln!(f, "Already waiting sigo {} '{}'", id, description)
            }
            SigoDisplay::BackTask(id, description) => {
                writeln!(f, "Returning sigo {} '{}'", id, description)
            }
            SigoDisplay::BackReadyTask(id, description) => {
                writeln!(f, "Already ready sigo {} '{}'", id, description)
            }
            SigoDisplay::AnnotateTask(id, description) => {
                writeln!(f, "Annotated sigo {} '{}'", id, description)
            }
            SigoDisplay::ListReadyTasks(tasks) => {
                writeln!(f, "{}", tasks_to_string(tasks))
            }
            SigoDisplay::ListWaitingTasks(tasks) => {
                writeln!(f, "{}", tasks_to_string(tasks))
            }
        }
    }
}

pub struct DisplaySimple<'a>(&'a SigoDisplay);
impl fmt::Display for DisplaySimple<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            SigoDisplay::CreateReadyTask(id) => {
                writeln!(
                    f,
                    "✅ Created sigo {0}.

    (use \"sigo done {0}\" to complete sigo)
    (use \"sigo wait {0}\" to change sigo waiting)",
                    id
                )
            }
            SigoDisplay::CreateWaitingTask(id) => {
                writeln!(
                    f,
                    "✅ Created waiting sigo {0}.

    (use \"sigo done {0}\" to complete sigo)
    (use \"sigo back {0}\" to change sigo ready)",
                    id
                )
            }
            SigoDisplay::ModifyTask(id, description) => {
                writeln!(
                    f,
                    "✅ Modify sigo {0} '{1}'.

    (use \"sigo done {0}\" to complete sigo)",
                    id, description
                )
            }
            SigoDisplay::CompleteTask(id, description) => {
                writeln!(
                    f,
                    "✅ Complete sigo {} '{}'.

    (use \"sigo list\" to list ready sigos)
    (use \"sigo add\" to add sigo)",
                    id, description
                )
            }
            SigoDisplay::WaitTask(id, description) => {
                writeln!(f, "✅ Waiting sigo {} '{}'.", id, description)
            }
            SigoDisplay::WaitWaitingTask(id, description) => {
                writeln!(f, "✅ Already waiting sigo {} '{}'.", id, description)
            }
            SigoDisplay::BackTask(id, description) => {
                writeln!(f, "✅ Returning sigo {} '{}'.", id, description)
            }
            SigoDisplay::BackReadyTask(id, description) => {
                writeln!(f, "✅ Already ready sigo {} '{}'.", id, description)
            }
            SigoDisplay::AnnotateTask(id, description) => {
                writeln!(f, "✅ Annotated sigo {} '{}'.", id, description)
            }
            SigoDisplay::ListReadyTasks(tasks) => {
                let tasks_len = tasks.len();
                if tasks_len == 0 {
                    writeln!(
                        f,
                        "No sigos! Woot woot!
    (use \"sigo add\" to add sigo)
    (use \"sigo waiting\" to list waiting sigos)"
                    )
                } else {
                    writeln!(
                        f,
                        "{}

{} sigos
    (use \"sigo done\" to complete sigo)
    (use \"sigo waiting\" to list waiting sigos)",
                        tasks_to_string(tasks),
                        tasks.len()
                    )
                }
            }
            SigoDisplay::ListWaitingTasks(tasks) => {
                let tasks_len = tasks.len();
                if tasks_len == 0 {
                    writeln!(
                        f,
                        "No matches.
    (use \"sigo add\" to add sigo)
    (use \"sigo list\" to list ready sigos)"
                    )
                } else {
                    writeln!(
                        f,
                        "{}

{} sigos
    (use \"sigo done\" to complete sigo)
    (use \"sigo list\" to list ready sigos)",
                        tasks_to_string(tasks),
                        tasks.len()
                    )
                }
            }
        }
    }
}
