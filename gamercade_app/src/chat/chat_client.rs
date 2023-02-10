use gamercade_protos::chat::{ChatChannel, ClientChatMessage};

use super::{
    chat_thread::{self, ChatThreadChannels},
    ChatRequestError, ReceivedChatMessage,
};

pub struct ChatClient {
    channels: ChatThreadChannels,
}

impl ChatClient {
    /// Initializes a chat client with only the
    /// global chat subscription
    pub fn new() -> Self {
        Self {
            channels: chat_thread::spawn(),
        }
    }

    pub fn send_chat_message(
        &self,
        chat_channel: ChatChannel,
        message: &str,
    ) -> Result<(), ChatRequestError> {
        self.channels
            .outbox
            .try_send(ClientChatMessage {
                chat_channel: Some(chat_channel),
                message_text: message.into(),
            })
            .map_err(|e| match e {
                tokio::sync::mpsc::error::TrySendError::Full(_) => {
                    ChatRequestError::TooManyRequests
                }
                tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                    ChatRequestError::ChannelClosed
                }
            })
    }

    pub fn subscribe_chat_channel(
        &self,
        chat_channel: ChatChannel,
    ) -> Result<(), ChatRequestError> {
        self.channels
            .pending_subscriptions
            .try_send(chat_channel)
            .map_err(|e| match e {
                tokio::sync::mpsc::error::TrySendError::Full(_) => {
                    ChatRequestError::TooManyRequests
                }
                tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                    ChatRequestError::ChannelClosed
                }
            })
    }

    pub fn get_new_messages(&mut self) -> Vec<ReceivedChatMessage> {
        let mut out = Vec::new();
        while let Ok(msg) = self.channels.incoming_messages_receiver.try_recv() {
            out.push(msg)
        }
        out
    }
}
