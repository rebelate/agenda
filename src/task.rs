use core::fmt;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Task {
    id: String,
    title: String,
    notes: Option<String>,
    status: String,
    updated: String,
    due: Option<String>,
}

impl Task {
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    pub fn title(&self) -> &str {
        self.title.as_ref()
    }

    pub fn notes(&self) -> Option<&String> {
        self.notes.as_ref()
    }

    pub fn status(&self) -> &str {
        self.status.as_ref()
    }

    pub fn updated(&self) -> &str {
        self.updated.as_ref()
    }

    pub fn due(&self) -> Option<&String> {
        self.due.as_ref()
    }
}
