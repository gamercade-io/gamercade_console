mod chat;
mod ips;

use chat::ChatClient;
use gamercade_interface::{
    chat::{chat_channel::Channel, ChatChannel},
    common::Empty,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test Chat Client:");

    let mut chat_client = ChatClient::new();

    chat_client
        .send_chat_message(
            ChatChannel {
                channel: Some(Channel::Global(Empty {})),
            },
            "Hello from client!",
        )
        .unwrap();

    chat_client
        .subscribe_chat_channel(ChatChannel {
            channel: Some(Channel::GameId(123)),
        })
        .unwrap();

    loop {
        chat_client.get_new_messages().iter().for_each(|msg| {
            let channel = match &msg.channel.channel {
                None => return,
                Some(Channel::Global(_)) => "global".to_string(),
                Some(Channel::GameId(game_id)) => game_id.to_string(),
                Some(Channel::RoomUuid(room_id)) => room_id.value.to_string(),
            };

            let username = match msg.message.user_id {
                0 => "system".to_string(),
                x => x.to_string(),
            };

            println!("{} @ {}: {}", username, channel, msg.message.message_text);
        });
    }
}
