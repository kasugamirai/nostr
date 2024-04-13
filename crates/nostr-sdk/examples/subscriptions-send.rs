use nostr_sdk::prelude::*;
use std::sync::{Arc, Mutex};
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let mut handles = vec![];
    let publish_success_count = Arc::new(Mutex::new(0));
    let publish_failed_count = Arc::new(Mutex::new(0));

    for i in 0..101 {
        let keys = Keys::generate();
        let public_key = keys.public_key();

        let client = Client::new(keys);
        //client.add_relay("wss://nostr.oxtr.dev").await.unwrap();
        client.add_relay("wss://relay.damus.io").await.unwrap();

        let subscription = Filter::new()
            .author(public_key)
            .kind(Kind::Metadata)
            .since(Timestamp::now());

        let success_count = Arc::clone(&publish_success_count);
        let failed_count = Arc::clone(&publish_failed_count);

        let handle = spawn(async move {
            client.connect().await;

            let sub_id = client.subscribe(vec![subscription], None).await;
            println!("Subscription ID: {:?}, {}", sub_id, i + 1);

            match client
                .publish_text_note("My first text note from Nostr SDK!", [])
                .await
            {
                Ok(_) => {
                    println!("Text note published successfully!");
                    let mut success = success_count.lock().unwrap();
                    *success += 1;
                }
                Err(e) => {
                    eprintln!("Failed to publish text note: {}", e);
                    let mut failed = failed_count.lock().unwrap();
                    *failed += 1;
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Success count: {}", *publish_success_count.lock().unwrap());
    println!("Failed count: {}", *publish_failed_count.lock().unwrap());

    Ok(())
}
