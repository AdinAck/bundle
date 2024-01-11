set -euxo pipefail

cd bundle

cargo build
cargo build --release
cargo clean

cd ../tests

cargo test
cargo clean
