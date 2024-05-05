use std::{
    collections::HashMap,
    fs,
    io::{self, Write},
    path::PathBuf,
};

use tabled::{
    grid::config::HorizontalLine,
    settings::{object::Rows, Padding, Theme},
    Table, Tabled,
};

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

pub fn create_file_if_not_exist(path: &PathBuf) -> io::Result<()> {
    if !path.is_file() {
        let mut f = fs::File::create(path)?;
        f.write_all(b"[]")?;
    }
    Ok(())
}
