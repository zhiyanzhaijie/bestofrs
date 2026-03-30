use domain::{Project, ProjectStatus};

pub fn parse_project_status(value: Option<&str>) -> ProjectStatus {
    match value.map(|value| value.trim().to_ascii_lowercase()) {
        Some(value) if value == "active" => ProjectStatus::Active,
        Some(value) if value == "disabled" => ProjectStatus::Disabled,
        Some(value) if value == "unknown" => ProjectStatus::Unknown,
        Some(_) | None => ProjectStatus::Unknown,
    }
}

pub fn project_status_value(status: ProjectStatus) -> &'static str {
    match status {
        ProjectStatus::Active => "Active",
        ProjectStatus::Disabled => "Disabled",
        ProjectStatus::Unknown => "Unknown",
    }
}

pub trait ProjectStatusExt {
    fn is_disabled(&self) -> bool;
}

impl ProjectStatusExt for Project {
    fn is_disabled(&self) -> bool {
        matches!(self.status, ProjectStatus::Disabled)
    }
}
