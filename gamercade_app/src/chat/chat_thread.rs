use tokio::{select, sync::mpsc};

use gamercade_protos::chat::{
    chat_service_client::ChatServiceClient, ChatChannel, ClientChatMessage,
};
use tokio_stream::StreamExt;

use crate::{chat::ReceivedChatMessage, ips::CHAT_IP};

pub struct ChatThreadChannels {
    pub outbox: mpsc::Sender<ClientChatMessage>,
    pub pending_subscriptions: mpsc::Sender<ChatChannel>,
    pub incoming_messages_receiver: mpsc::UnboundedReceiver<ReceivedChatMessage>,
}

pub fn spawn() -> ChatThreadChannels {
    let (outbox, mut outbox_receiver) = mpsc::channel(super::MAX_CONCURRENT_MESSAGES);
    let (pending_subscriptions, mut pending_subscriptions_receiver) =
        mpsc::channel(super::MAX_CONCURRENT_MESSAGES);
    let (incoming_messages, incoming_messages_receiver) = mpsc::unbounded_channel();

    // Subscribe to global chat
    pending_subscriptions
        .try_send(ChatChannel {
            game_id: 0,
            room_uuid: None,
        })
        .unwrap();

    tokio::spawn(async move {
        // Initialize the chat client
        let Ok(mut client) = ChatServiceClient::connect(CHAT_IP).await else {
            //TODO: Add retry mechanism or atomic bool for connection status?
            println!("Failed to connect to chat service!");
            return;
        };

        loop {
            select! {

                out = outbox_receiver.recv() => {
                    if let Some(out) = out {
                        if let Err(e) = client.send_chat_message(out).await {
                            println!("{}", e)
                        }
                    }
                },

                sub = pending_subscriptions_receiver.recv() => {
                    if let Some(sub) = sub {
                        let channel_name = sub.clone();
                        match client.subscribe_chat_channel(sub).await {
                            Ok(new_channel) => {
                                let mut new_channel = new_channel.into_inner();
                                let sender = incoming_messages.clone();
                                tokio::spawn(async move {
                                    while let Some(Ok(message)) = new_channel.next().await {
                                        let channel_name = channel_name.clone();
                                        let _ = sender.send(ReceivedChatMessage { message, channel: channel_name });
                                    }
                                });
                            },
                            Err(e) => {
                                println!("{}", e);
                            }
                        }
                    }
                }
            }
        }
    });

    ChatThreadChannels {
        outbox,
        pending_subscriptions,
        incoming_messages_receiver,
    }
}
