use bevy::prelude::*;
use chrono::{DateTime, Utc};
use std::fmt::Debug;
use uuid::Uuid;



#[derive(Component, Debug, Clone)]
pub struct Comment {
    pub uuid: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
    pub author: String,
    pub is_resolved: bool,
}

impl Default for Comment {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            text: String::new(),
            created_at: Utc::now(),
            author: String::from("Anonymous"),
            is_resolved: false,
        }
    }
}

impl Comment {
    pub fn new(text: &str, author: &str) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            text: text.into(),
            created_at: Utc::now(),
            author: author.into(),
            is_resolved: false,
        }
    }

    pub fn edit(&mut self, text: String) {
        self.text = text;
    }
}


