#!/usr/bin/env bash
cargo build --release  --features cloudinary,production
cargo install sqlx-cli --no-default-features --features native-tls,postgres
sqlx migrate run
