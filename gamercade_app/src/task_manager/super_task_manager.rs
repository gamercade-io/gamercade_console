use tokio::sync::mpsc::{channel, Receiver};

use crate::local_directory::{Tag, TagId};

use super::{TagManager, SUPER_TASK_CHANNEL_SIZE};

#[derive(Debug)]
pub enum TaskNotification {
    GlobalTags(Vec<(TagId, Tag)>),
}

pub struct SuperTaskManager {
    pub events: Receiver<TaskNotification>,
    pub tags: TagManager,
}

impl Default for SuperTaskManager {
    fn default() -> Self {
        let (event_tx, events) = channel(SUPER_TASK_CHANNEL_SIZE);

        Self {
            tags: TagManager::new(event_tx),
            events,
        }
    }
}
