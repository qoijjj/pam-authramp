test-auth:
  cargo build --release
  cargo test --release --test "*"