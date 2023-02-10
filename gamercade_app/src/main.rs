use tokio_stream::StreamExt;

use gamercade_protos::chat::chat_service_client::ChatServiceClient;
use gamercade_protos::chat::{ChatChannel, ClientChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test Chat Client:");

    let mut client = ChatServiceClient::connect("http://127.0.0.1:50051").await?;

    client
        .send_chat_message(ClientChatMessage {
            message_text: "Hello from client".to_string(),
            chat_channel: None,
        })
        .await
        .unwrap();

    let mut stream = client
        .subscribe_chat_channel(ChatChannel {
            game_uuid: None,
            channel_uuid: None,
        })
        .await
        .unwrap()
        .into_inner();

    while let Some(Ok(msg)) = stream.next().await {
        println!("got message: {}", msg.message_text)
    }

    Ok(())
}
