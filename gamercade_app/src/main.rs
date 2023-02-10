mod chat;
mod ips;

use chat::ChatClient;
use gamercade_protos::chat::ChatChannel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test Chat Client:");

    let mut chat_client = ChatClient::new();

    chat_client
        .send_chat_message(
            ChatChannel {
                game_uuid: None,
                room_uuid: None,
            },
            "Hello from client!",
        )
        .unwrap();

    loop {
        chat_client
            .get_new_messages()
            .iter()
            .for_each(|msg| println!("got msg: {}", msg.message.message_text))
    }
}
