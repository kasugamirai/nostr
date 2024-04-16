use std::str::FromStr;

use nostr_sdk::{Client, EventBuilder, Keys, Kind, PublicKey, Tag};
#[tokio::main]
async fn main() {
    let key = Keys::generate();
    let client = Client::new(key.clone());
    let public_key =
        PublicKey::from_str("npub1080l37pfvdpyuzasyuy2ytjykjvq3ylr5jlqlg7tvzjrh9r8vn3sf5yaph")
            .unwrap();
    client.add_relay("wss://nostr.oxtr.dev").await.unwrap();
    client.connect().await;
    let kind = Kind::EncryptedDirectMessage;
    let content = "Hello, World!";
    let tags = vec![Tag::public_key(public_key)];
    let builder = EventBuilder::new(kind, content, tags);
    match client.send_event_builder(builder).await {
        Ok(_) => {
            println!("Event sent successfully!");
        }
        Err(e) => {
            println!("Failed to send event: {}", e);
        }
    }
}
