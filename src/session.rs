use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;

use serde::{Deserialize, Serialize};

/// Source of a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionSource {
    Interactive,
    Resume,
    Fork,
    Api,
    Unknown,
}

/// A single chat message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMsg {
    pub role: String,
    pub content: String,
}

/// A thread representing a conversation session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: String,
    pub name: Option<String>,
    pub preview: String,
    pub ephemeral: bool,
    pub active: bool,
    pub source: SessionSource,
    pub created_at: i64,
    pub updated_at: i64,
    pub messages: Vec<ChatMsg>,
}

/// A snapshot for undo operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub id: usize,
    pub messages: Vec<ChatMsg>,
    pub timestamp: i64,
}

/// Manages sessions, threads, and undo history.
pub struct SessionStore {
    threads: HashMap<String, Thread>,
    snapshots: HashMap<String, Vec<Snapshot>>,
    data_dir: PathBuf,
}

impl SessionStore {
    pub fn new(data_dir: PathBuf) -> Self {
        fs::create_dir_all(&data_dir).ok();
        Self {
            threads: HashMap::new(),
            snapshots: HashMap::new(),
            data_dir,
        }
    }

    pub fn create_thread(
        &mut self,
        source: SessionSource,
        ephemeral: bool,
    ) -> &mut Thread {
        let id = format!("thread-{}", uuid::Uuid::new_v4());
        let now = chrono::Utc::now().timestamp();
        let thread = Thread {
            id: id.clone(),
            name: None,
            preview: "New conversation".to_string(),
            ephemeral,
            active: true,
            source,
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
        };
        self.threads.insert(id.clone(), thread);
        self.threads.get_mut(&id).unwrap()
    }

    pub fn get_thread(&self, id: &str) -> Option<&Thread> {
        self.threads.get(id)
    }

    pub fn get_thread_mut(&mut self, id: &str) -> Option<&mut Thread> {
        self.threads.get_mut(id)
    }

    pub fn list_active_threads(&self) -> Vec<&Thread> {
        self.threads
            .values()
            .filter(|t| t.active)
            .collect()
    }

    pub fn push_snapshot(&mut self, thread_id: &str, messages: &[ChatMsg]) {
        let snapshots = self.snapshots.entry(thread_id.to_string()).or_default();
        let id = snapshots.len();
        let snapshot = Snapshot {
            id,
            messages: messages.to_vec(),
            timestamp: chrono::Utc::now().timestamp(),
        };
        snapshots.push(snapshot);
    }

    pub fn pop_snapshot(&mut self, thread_id: &str) -> Option<Snapshot> {
        self.snapshots.get_mut(thread_id)?.pop()
    }

    pub fn restore_snapshot(&self, thread_id: &str, snapshot_id: usize) -> Option<&Snapshot> {
        self.snapshots.get(thread_id)?.iter().find(|s| s.id == snapshot_id)
    }

    pub fn persist_thread(&self, thread: &Thread) -> std::io::Result<()> {
        let file_path = self.data_dir.join(format!("{}.json", thread.id));
        let json = serde_json::to_string_pretty(thread)?;
        fs::write(file_path, json)
    }

    pub fn load_thread(&mut self, id: &str) -> std::io::Result<Option<&Thread>> {
        let file_path = self.data_dir.join(format!("{}.json", id));
        if !file_path.exists() {
            return Ok(None);
        }
        let json = fs::read_to_string(&file_path)?;
        let thread: Thread = serde_json::from_str(&json)?;
        self.threads.insert(id.to_string(), thread);
        Ok(self.threads.get(id))
    }

    pub fn archive_thread(&mut self, id: &str) -> bool {
        if let Some(thread) = self.threads.get_mut(id) {
            thread.active = false;
            thread.updated_at = chrono::Utc::now().timestamp();
            true
        } else {
            false
        }
    }

    pub fn unarchive_thread(&mut self, id: &str) -> bool {
        if let Some(thread) = self.threads.get_mut(id) {
            thread.active = true;
            thread.updated_at = chrono::Utc::now().timestamp();
            true
        } else {
            false
        }
    }

    pub fn trim_snapshots(&mut self, thread_id: &str, keep: usize) {
        if let Some(snapshots) = self.snapshots.get_mut(thread_id) {
            if snapshots.len() > keep {
                let drain = snapshots.len() - keep;
                snapshots.drain(0..drain);
            }
        }
    }
}