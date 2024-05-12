use std::{error::Error, fmt, path::PathBuf};

pub type Result<T> = std::result::Result<T, SigoError>;

#[derive(Debug)]
pub enum SigoError {
    FileCreateErr(PathBuf, std::io::Error),
    FileReadErr(PathBuf, std::io::Error),
    FileWriteErr(PathBuf, std::io::Error),
    FileRenameErr(PathBuf, PathBuf, std::io::Error),
    ParseStrToTasksErr(PathBuf, serde_json::Error),
    ParseTasksToStrErr(serde_json::Error),
    TaskNotFound(u32),
}

impl fmt::Display for SigoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigoError::FileCreateErr(path, ref err) => {
                writeln!(f, "unable to create file {:?}: {}", path, err)
            }
            SigoError::FileReadErr(path, ref err) => {
                writeln!(f, "unable to read file {:?}: {}", path, err)
            }
            SigoError::FileWriteErr(path, ref err) => {
                writeln!(f, "unable to write file {:?}: {}", path, err)
            }
            SigoError::FileRenameErr(srcpath, tarpath, ref err) => writeln!(
                f,
                "unable to rename file {:?} to file {:?}: {}",
                srcpath, tarpath, err
            ),
            SigoError::ParseStrToTasksErr(path, ref err) => {
                writeln!(f, "unbale to parse file {:?}: {}", path, err)
            }
            SigoError::ParseTasksToStrErr(err) => writeln!(f, "unbale to parse sigo {}", err),
            SigoError::TaskNotFound(id) => writeln!(f, "not found sigo {}", id),
        }
    }
}

impl Error for SigoError {}

impl From<serde_json::Error> for SigoError {
    fn from(value: serde_json::Error) -> Self {
        SigoError::ParseTasksToStrErr(value)
    }
}
