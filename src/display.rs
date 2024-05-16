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
    ListReadyTasks(String),
    ListWaitingTasks(String),
}

use std::fmt;

impl SigoDisplay {
    pub fn display_minimun(&self) -> DisplayMinimum {
        DisplayMinimum(self)
    }

    // pub fn display_simple(&self) -> DisplaySimple {
    //     DisplaySimple(self)
    // }
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
            SigoDisplay::ListReadyTasks(tasks_str) => {
                writeln!(f, "{}", tasks_str)
            }
            SigoDisplay::ListWaitingTasks(tasks_str) => {
                writeln!(f, "{}", tasks_str)
            }
        }
    }
}
