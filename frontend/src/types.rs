use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoPriority {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}

impl TodoPriority {
    pub fn as_str(&self) -> &'static str {
        match self {
            TodoPriority::Low => "low",
            TodoPriority::Medium => "medium",
            TodoPriority::High => "high",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "low" => TodoPriority::Low,
            "medium" => TodoPriority::Medium,
            "high" => TodoPriority::High,
            _ => TodoPriority::Medium,
        }
    }

    pub fn display(&self) -> &'static str {
        match self {
            TodoPriority::Low => "Low",
            TodoPriority::Medium => "Medium",
            TodoPriority::High => "High",
        }
    }

    #[allow(dead_code)]
    pub fn all() -> &'static [TodoPriority] {
        &[TodoPriority::High, TodoPriority::Medium, TodoPriority::Low]
    }
}

impl std::fmt::Display for TodoPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub completed: bool,
    pub priority: TodoPriority,
    pub category: String,
    #[serde(rename = "dueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatusState {
    Idle,
    Syncing,
    Success,
    Error,
}

#[derive(Debug, Clone)]
pub struct SyncStatus {
    pub status: SyncStatusState,
    pub last_synced_at: Option<String>,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            status: SyncStatusState::Idle,
            last_synced_at: None,
        }
    }
}

pub const DEFAULT_CATEGORIES: &[&str] = &["Personal", "Work", "Shopping", "Ideas", "Urgent"];

#[derive(Debug, Clone)]
pub struct NewTodoData {
    pub title: String,
    pub description: Option<String>,
    pub priority: TodoPriority,
    pub category: String,
    pub due_date: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TodoUpdate {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub completed: Option<bool>,
    pub priority: Option<TodoPriority>,
    pub category: Option<String>,
    pub due_date: Option<Option<String>>,
}

pub fn format_date_short(date_str: &str) -> String {
    // Simple format: "Jan 5" style
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() < 3 {
        return date_str.to_string();
    }
    let month = match parts[1].parse::<u32>().unwrap_or(0) {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => "???",
    };
    let day = parts[2].parse::<u32>().unwrap_or(0);
    format!("{} {}", month, day)
}

pub fn format_full_datetime(date_str: &str) -> String {
    // Simple format: "Jul 5, 3:42 PM" style
    if let Some(space_pos) = date_str.find('T') {
        let date_part = &date_str[..space_pos];
        let time_part = &date_str[space_pos + 1..];
        let time_short = if let Some(pos) = time_part.find('+') {
            &time_part[..pos]
        } else if let Some(pos) = time_part.find('Z') {
            &time_part[..pos]
        } else {
            time_part
        };
        let formatted_date = format_date_short(date_part);
        let time_components: Vec<&str> = time_short.split(':').collect();
        if time_components.len() >= 2 {
            let hour: u32 = time_components[0].parse().unwrap_or(0);
            let min = time_components[1];
            let (h, ampm) = if hour == 0 {
                (12, "AM")
            } else if hour < 12 {
                (hour, "AM")
            } else if hour == 12 {
                (12, "PM")
            } else {
                (hour - 12, "PM")
            };
            format!("{}, {}:{:>02} {}", formatted_date, h, min, ampm)
        } else {
            formatted_date
        }
    } else {
        format_date_short(date_str)
    }
}

pub fn format_time_short(date_str: &str) -> String {
    if let Some(space_pos) = date_str.find('T') {
        let time_part = &date_str[space_pos + 1..];
        let time_short = if let Some(pos) = time_part.find('+') {
            &time_part[..pos]
        } else if let Some(pos) = time_part.find('Z') {
            &time_part[..pos]
        } else {
            time_part
        };
        let time_components: Vec<&str> = time_short.split(':').collect();
        if time_components.len() >= 2 {
            let hour: u32 = time_components[0].parse().unwrap_or(0);
            let min = time_components[1];
            let (h, ampm) = if hour == 0 {
                (12, "AM")
            } else if hour < 12 {
                (hour, "AM")
            } else if hour == 12 {
                (12, "PM")
            } else {
                (hour - 12, "PM")
            };
            format!("{}:{:>02} {}", h, min, ampm)
        } else {
            time_short.to_string()
        }
    } else {
        date_str.to_string()
    }
}

pub fn current_time_string() -> String {
    let date = js_sys::Date::new_0();
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    let seconds = date.get_seconds();
    let (h, ampm) = if hours == 0 {
        (12, "AM")
    } else if hours < 12 {
        (hours as u32, "AM")
    } else if hours == 12 {
        (12, "PM")
    } else {
        ((hours - 12) as u32, "PM")
    };
    format!("{}:{:>02}:{:>02} {}", h, minutes, seconds, ampm)
}

pub fn is_date_overdue(due_date: &str) -> bool {
    let now = js_sys::Date::new_0();
    let year = now.get_full_year();
    let month = now.get_month() + 1; // 0-indexed
    let day = now.get_date();

    let parts: Vec<&str> = due_date.split('-').collect();
    if parts.len() < 3 {
        return false;
    }
    let due_year: u32 = parts[0].parse().unwrap_or(0);
    let due_month: u32 = parts[1].parse().unwrap_or(0);
    let due_day: u32 = parts[2].parse().unwrap_or(0);

    (due_year as i32) < (year as i32)
        || ((due_year as i32) == (year as i32) && due_month < month)
        || ((due_year as i32) == (year as i32) && due_month == month && due_day < day)
}

pub fn current_date_display() -> String {
    let date = js_sys::Date::new_0();
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let day_name = days[date.get_day() as usize];
    let month_name = months[date.get_month() as usize];
    let day_num = date.get_date();
    format!("{}, {} {}", day_name, month_name, day_num)
}
