mod chat;
mod ips;

use chat::ChatClient;
use gamercade_interface::chat::ChatChannel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test Chat Client:");

    let mut chat_client = ChatClient::new();

    chat_client
        .send_chat_message(
            ChatChannel {
                game_id: 0,
                room_uuid: None,
            },
            "Hello from client!",
        )
        .unwrap();

    chat_client
        .subscribe_chat_channel(ChatChannel {
            game_id: 123,
            room_uuid: None,
        })
        .unwrap();

    loop {
        chat_client.get_new_messages().iter().for_each(|msg| {
            let channel = match msg.channel.game_id {
                0 => "global".to_string(),
                x => x.to_string(),
            };

            let username = match msg.message.user_id {
                0 => "system".to_string(),
                x => x.to_string(),
            };

            println!("{} @ {}: {}", username, channel, msg.message.message_text);
        });
    }
}
