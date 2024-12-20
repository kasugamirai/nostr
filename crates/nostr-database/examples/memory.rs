// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::time::Duration;

use nostr::prelude::*;
use nostr::{EventBuilder, Filter, Keys, Kind, Metadata, Tag};
use nostr_database::memory::MemoryDatabase;
use nostr_database::{MemoryDatabaseOptions, NostrDatabase};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let secret_key =
        SecretKey::from_bech32("nsec1j4c6269y9w0q2er2xjw8sv2ehyrtfxq3jwgdlxj6qfn8z4gjsq5qfvfk99")
            .unwrap();
    let keys_a = Keys::new(secret_key);

    let secret_key =
        SecretKey::from_bech32("nsec1ufnus6pju578ste3v90xd5m2decpuzpql2295m3sknqcjzyys9ls0qlc85")
            .unwrap();
    let keys_b = Keys::new(secret_key);

    let opts = MemoryDatabaseOptions {
        events: true,
        ..Default::default()
    };
    let database = MemoryDatabase::with_opts(opts);

    for i in 0..100_000 {
        let event = EventBuilder::text_note(format!("Event #{i}"))
            .sign_with_keys(&keys_a)
            .unwrap();
        database.save_event(&event).await.unwrap();

        let event = EventBuilder::text_note(format!("Reply to event #{i}"))
            .tags([Tag::event(event.id), Tag::public_key(event.pubkey)])
            .sign_with_keys(&keys_b)
            .unwrap();
        database.save_event(&event).await.unwrap();
    }

    for i in 0..10 {
        let metadata = Metadata::new().name(format!("Name #{i}"));
        let event = EventBuilder::metadata(&metadata)
            .sign_with_keys(&keys_a)
            .unwrap();
        database.save_event(&event).await.unwrap();
    }

    for i in 0..500_000 {
        let event = EventBuilder::new(Kind::Custom(123), "Custom with d tag")
            .tag(Tag::identifier(format!("myid{i}")))
            .sign_with_keys(&keys_a)
            .unwrap();
        database.save_event(&event).await.unwrap();
    }

    let events = database
        .query(vec![Filter::new()
            .kinds(vec![Kind::Metadata, Kind::Custom(123), Kind::TextNote])
            .limit(20)
            //.kind(Kind::Custom(123))
            //.identifier("myid5000")
            .author(keys_a.public_key())])
        .await
        .unwrap();
    println!("Got {} events", events.len());

    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
