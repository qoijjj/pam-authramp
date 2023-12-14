unit-test:
  cargo test --lib

auth-test:
  cargo build --release
  cargo test --release --test "*"