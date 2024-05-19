use std::{collections::HashMap, fs, io::Write, path::PathBuf};

use tabled::{
    grid::{config::{ColoredConfig, Entity, HorizontalLine}, records::{vec_records::{CellInfo, VecRecords}, ExactRecords, Records}},
    settings::{
        object::{Columns, Rows}, Alignment, CellOption, Color, Format, Modify, Padding, Theme, Width
    },
    Table, Tabled,
};
use terminal_size::{terminal_size, Height as TerminalHeight, Width as TerminalWidth};

use crate::{error::SigoError, Priority};

// TODO priority high is bold, low is toumei
// this could be macro, i donot know...
pub fn tasks_to_string<I, T>(tasks: I) -> String
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    let (width, _height) = get_terminal_size();
    let mut style = Theme::default();
    style.set_lines_horizontal(HashMap::from_iter([(
        1,
        HorizontalLine::full('-', ' ', ' ', ' '),
    )]));
    style.set_border_intersection_top(' ');
    Table::new(tasks)
        .modify(Rows::new(..), Padding::new(0, 0, 0, 0))
        .with(Modify::new(Columns::single(0)).with(Alignment::right()))
        .with(Modify::new(Columns::single(2)).with(Width::wrap(width / 2)))
        .with(Modify::new(Rows::new(1..)).with(PriorityColorization))
        .with(style)
        .to_string()
}

#[derive(Clone)]
struct PriorityColorization;

impl CellOption<VecRecords<CellInfo<String>>, ColoredConfig> for PriorityColorization {
    fn change(
        self,
        records: &mut VecRecords<CellInfo<String>>,
        cfg: &mut ColoredConfig,
        entity: Entity,
    ) {
        let (count_rows, count_cols) = (records.count_rows(), records.count_columns());

        for (row, col) in entity.iter(count_rows, count_cols) {
            let priority = records[row][1].as_ref();
            let color = priority_color(priority);
            cfg.set_color(Entity::Cell(row, col), color.into());
        }
    }
}

fn priority_color(priority: &str) -> Color {
    match priority {
        "H" => Color::BOLD,
        "M" => Color::default(),
        "L" => Color::new("\u{1b}[2m", "\u{1b}[22m"),
        _ => Color::default(),
    }
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

pub fn display_vec_string(v: &[String]) -> String {
    v.join("\n* ")
}

pub fn display_option_priority(o: &Option<Priority>) -> String {
    match o {
        Some(p) => p.to_string(),
        None => "".to_string(),
    }
}

fn get_terminal_size() -> (usize, usize) {
    let (TerminalWidth(width), TerminalHeight(height)) =
        terminal_size().expect("failed to obtain a terminal size");

    (width as usize, height as usize)
}
