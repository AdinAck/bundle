set -euxo pipefail

cargo build
cargo build --release
cargo test -p bundle-tests
cargo clean
