#!/bin/bash

set -euo pipefail

# MSRV
msrv="1.64.0"

is_msrv=false
version=""

# Check if "msrv" is passed as an argument
if [[ "$#" -gt 0 && "$1" == "msrv" ]]; then
    is_msrv=true
    version="+$msrv"
fi

# Check if MSRV
if [ "$is_msrv" == true ]; then
    # Install MSRV
    rustup install $msrv
    rustup component add clippy --toolchain $msrv
    rustup target add wasm32-unknown-unknown --toolchain $msrv
fi

buildargs=(
    "-p nostr"
    "-p nostr --no-default-features --features alloc"
    "-p nostr --no-default-features --features alloc,all-nips"
    "-p nostr --features blocking"
    "-p nostr-database"
    "-p nostr-zapper"
    "-p nostr-sdk"
    "-p nostr-sdk --no-default-features"
    "-p nostr-sdk --features nip47,nip57"
    "-p nostr-sdk --features nip47,nip57 --target wasm32-unknown-unknown"
    "-p nostr-sdk --features indexeddb,webln --target wasm32-unknown-unknown"
    "-p nostr-sdk --features sqlite"
    "-p nostr-sdk --features ndb"
)

skip_msrv=(
  "-p nostr-sdk --features ndb" # MSRV: 1.70.0
)

for arg in "${buildargs[@]}";
do
    # Skip the current crate if is_msrv is true and it's in the skip list
    if [ "$is_msrv" == true ] && [[ "${skip_msrv[*]}" =~ $arg ]]; then
        echo "Skipping MSRV check for '$arg'"
        echo
        continue
    fi

    if [[ $version == "" ]];
    then
        echo  "Checking '$arg' [default]"
    else
        echo  "Checking '$arg' [$version]"
    fi

    cargo $version check $arg

    if [[ $arg != *"--target wasm32-unknown-unknown"* ]];
    then
        cargo $version test $arg
    fi

    cargo $version clippy $arg -- -D warnings

    echo
done