use domain::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TagDto {
    pub label: String,
    pub value: String,
}

impl From<Tag> for TagDto {
    fn from(value: Tag) -> Self {
        Self {
            label: value.label.as_str().to_string(),
            value: value.value.as_str().to_string(),
        }
    }
}
