use std::fmt::Error;
use chrono::prelude::*;
use std::sync::{Arc, Mutex};

use crate::models::replay::Replay;

pub struct Database {
    pub todos: Arc<Mutex<Vec<Replay>>>,
}

impl Database {
    pub fn new() -> Self {
        let todos = Arc::new(Mutex::new(vec![]));
        Database { todos }
    }

    pub fn create_replay(&self, replay: Replay) -> Result<Replay, Error> {
        let mut replays = self.todos.lock().unwrap();
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now();
        let updated_at = Utc::now();
        let replay = Replay {
            id: Some(id),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            ..replay
        };
        replays.push(replay.clone());
        Ok(replay)
    }
}