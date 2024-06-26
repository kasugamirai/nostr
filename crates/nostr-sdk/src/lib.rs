// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! High level Nostr client library.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rustdoc::bare_urls)]
#![allow(unknown_lints)]
#![allow(clippy::arc_with_non_send_sync)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(feature = "all-nips", doc = include_str!("../README.md"))]

#[cfg(all(target_arch = "wasm32", feature = "blocking"))]
compile_error!("`blocking` feature can't be enabled for WASM targets");

#[doc(hidden)]
pub use async_utility;
#[doc(hidden)]
pub use nostr::{self, *};
#[doc(hidden)]
pub use nostr_database::{self as database, NostrDatabase, NostrDatabaseExt, Profile};
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", feature = "indexeddb"))]
pub use nostr_indexeddb::{IndexedDBError, WebDatabase};
#[doc(hidden)]
#[cfg(feature = "ndb")]
pub use nostr_ndb::{self as ndb, NdbDatabase};
#[doc(hidden)]
pub use nostr_relay_pool::{
    self as pool, AtomicRelayServiceFlags, FilterOptions, NegentropyDirection, NegentropyOptions,
    Relay, RelayConnectionStats, RelayOptions, RelayPool, RelayPoolNotification, RelayPoolOptions,
    RelaySendOptions, RelayServiceFlags, RelayStatus, SubscribeAutoCloseOptions, SubscribeOptions,
};
#[doc(hidden)]
#[cfg(feature = "rocksdb")]
pub use nostr_rocksdb::RocksDatabase;
#[doc(hidden)]
pub use nostr_signer::{self as signer, NostrSigner, NostrSignerType};
#[doc(hidden)]
#[cfg(feature = "sqlite")]
pub use nostr_sqlite::{Error as SQLiteError, SQLiteDatabase};
#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", feature = "webln"))]
pub use nostr_webln::WebLNZapper;
#[doc(hidden)]
#[cfg(feature = "nip57")]
pub use nostr_zapper::{self as zapper, NostrZapper, ZapperBackend, ZapperError};
#[doc(hidden)]
#[cfg(feature = "nip47")]
pub use nwc::{self, NostrWalletConnectOptions, NWC};
#[doc(hidden)]
#[cfg(feature = "blocking")]
use once_cell::sync::Lazy;
#[doc(hidden)]
#[cfg(feature = "blocking")]
use tokio::runtime::Runtime;
#[doc(hidden)]
#[cfg(feature = "blocking")]
pub use tokio::task::spawn_blocking;

pub mod client;
pub mod prelude;

pub use self::client::{Client, ClientBuilder, Options};

#[doc(hidden)]
#[cfg(feature = "blocking")]
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));

#[doc(hidden)]
#[allow(missing_docs)]
#[cfg(feature = "blocking")]
pub fn block_on<F>(future: F) -> F::Output
where
    F: core::future::Future,
{
    RUNTIME.block_on(future)
}
