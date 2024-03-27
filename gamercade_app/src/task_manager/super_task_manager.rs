use tokio::sync::mpsc::{channel, Receiver};

use crate::local_directory::{PermissionLevel, PermissionLevelId, Tag, TagId};

use super::{AuthorManager, TagManager, SUPER_TASK_CHANNEL_SIZE};

#[derive(Debug)]
pub enum TaskNotification {
    GlobalTags(Vec<(TagId, Tag)>),
    GlobalPermissionLevels(Vec<(PermissionLevelId, PermissionLevel)>),
}

pub struct SuperTaskManager {
    pub events: Receiver<TaskNotification>,
    pub tags: TagManager,
    pub author: AuthorManager,
}

impl Default for SuperTaskManager {
    fn default() -> Self {
        let (event_tx, events) = channel(SUPER_TASK_CHANNEL_SIZE);

        Self {
            tags: TagManager::new(event_tx.clone()),
            author: AuthorManager::new(event_tx.clone()),
            events,
        }
    }
}
