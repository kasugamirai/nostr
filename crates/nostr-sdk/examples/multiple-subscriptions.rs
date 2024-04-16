use nostr_sdk::{bitcoin::XKeyIdentifier, prelude::*};
use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt::init();

    let mut notification_handles = vec![];
    //let publish_success_count = Arc::new(Mutex::new(0));
    //let publish_failed_count = Arc::new(Mutex::new(0));
    let mut saved_keys = vec![];

    for i in 0..100 {
        let keys = Keys::generate();
        saved_keys.push(keys.clone());
        let public_key = keys.public_key();
        println!("Public key: {}", public_key);

        let client = Client::new(&keys);
        client.add_relay("wss://nostr.oxtr.dev").await.unwrap();
        //client.add_relay("wss://relay.damus.io").await.unwrap();

        client.connect().await;

        let subscription = Filter::new().author(public_key).kind(Kind::Metadata);

        //let success_count = Arc::clone(&publish_success_count);
        //let failed_count = Arc::clone(&publish_failed_count);

        let sub_id = client.subscribe(vec![subscription], None).await;
        println!("Subscription ID: {:?}, {}", sub_id.clone(), i + 1);

        let notification_handle = spawn(async move {
            client
                .handle_notifications(|notification| {
                    handle_event(notification, sub_id.clone(), keys.clone())
                })
                .await
                .unwrap();
        });
        notification_handles.push(notification_handle);
    }

    //for handle in notification_handles {
    //    handle.await.unwrap();
    //}

    println!("All subscriptions are done!");

    for key in saved_keys {
        let client = Client::new(key.clone());
        let public_key = key.public_key();
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

    //println!("Success count: {}", *publish_success_count.lock().unwrap());
    //println!("Failed count: {}", *publish_failed_count.lock().unwrap());

    Ok(())
}

async fn handle_event(
    notification: RelayPoolNotification,
    sub_id_1: SubscriptionId,
    keys: Keys,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let RelayPoolNotification::Event {
        subscription_id,
        event,
        ..
    } = notification
    {
        // Check subscription ID
        if subscription_id == sub_id_1 {
            // Handle (ex. update specific UI)
        }

        // Check kind
        if event.kind() == Kind::EncryptedDirectMessage {
            if let Ok(msg) = nip04::decrypt(keys.secret_key()?, event.author_ref(), event.content())
            {
                println!("DM: {msg}");
            } else {
                tracing::error!("Impossible to decrypt direct message");
            }
        } else if event.kind() == Kind::TextNote {
            println!("TextNote: {:?}", event);
        } else {
            println!("{:?}", event);
        }
    }
    Ok(false) // Set to true to exit from the loop
}
