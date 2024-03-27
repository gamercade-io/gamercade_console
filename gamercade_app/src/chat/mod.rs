mod chat_client;
mod chat_thread;

pub use chat_client::ChatClient;
use gamercade_interface::chat::{ChatChannel, ServerChatMessage};

pub const MAX_CONCURRENT_MESSAGES: usize = 8;

pub struct ReceivedChatMessage {
    pub message: ServerChatMessage,
    pub channel: ChatChannel,
}

#[derive(Debug)]
pub enum ChatRequestError {
    TooManyRequests,
    ChannelClosed,
}
