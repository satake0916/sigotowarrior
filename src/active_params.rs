use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;
use tabled::Tabled;

use crate::utils;
use crate::Priority;

#[derive(Debug, Tabled, Serialize, Deserialize, Clone)]
pub struct ActiveParams {
    pub id: u32,
    #[tabled(rename = "P", display_with = "utils::display_option_priority")]
    pub priority: Option<Priority>,
    #[tabled(display_with = "utils::display_vec_string")]
    pub description: Vec<String>,
    #[tabled(display_with = "utils::display_option_date")]
    pub due: Option<NaiveDate>,
}

impl ActiveParams {
    pub fn get_primary_description(&self) -> String {
        self.description
            .first()
            .unwrap_or(&"".to_owned())
            .to_string()
    }

    pub fn annotate_description(&self, text: &str) -> Self {
        let mut description = self.description.clone();
        description.push(text.to_string());
        Self {
            id: self.id,
            description,
            priority: self.priority,
            due: self.due,
        }
    }

    pub fn modify_priority(&self, priority: Option<Priority>) -> Self {
        Self {
            id: self.id,
            description: self.description.clone(),
            priority: priority.or(self.priority),
            due: self.due,
        }
    }

    pub fn modify_due(&self, due: Option<NaiveDate>) -> Self {
        Self {
            id: self.id,
            description: self.description.clone(),
            priority: self.priority,
            due: due.or(self.due),
        }
    }
}

impl Ord for ActiveParams {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self.priority, other.priority) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (Some(_), None) => std::cmp::Ordering::Less,
            (Some(x), Some(y)) => x.cmp(&y),
        }
    }
}

impl PartialOrd for ActiveParams {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ActiveParams {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.description == other.description
            && self.priority == other.priority
    }
}

impl Eq for ActiveParams {}
