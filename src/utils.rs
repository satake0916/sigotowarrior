use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use tabled::{
    grid::config::HorizontalLine,
    settings::{object::Rows, Padding, Theme},
    Table, Tabled,
};

use crate::error::SigoError;

pub fn tasks_to_string<I, T>(tasks: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    let mut style = Theme::default();
    style.set_lines_horizontal(HashMap::from_iter([(
        1,
        HorizontalLine::full('-', ' ', ' ', ' '),
    )]));
    style.set_border_intersection_top(' ');
    Table::new(tasks)
        .modify(Rows::new(..), Padding::new(0, 0, 0, 0))
        .with(style)
        .to_string()
}

pub fn create_file_if_not_exist(path: &PathBuf) -> Result<(), SigoError> {
    if !path.is_file() {
        let mut f =
            fs::File::create(path).map_err(|e| SigoError::FileCreateErr(path.to_path_buf(), e))?;
        f.write_all(b"[]")
            .map_err(|e| SigoError::FileWriteErr(path.to_path_buf(), e))?;
    }
    Ok(())
}
