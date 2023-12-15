# Dev Commands

build:
  @echo 'Building in release mode'
  cargo build --release

test:
  @echo 'Building & running all tests'
  just build
  cargo test --release

unit-test:
  @echo 'Running unit tests'
  cargo test --lib

auth-test:
  @echo 'Running auth integration tests'
  just build
  cargo test --release --test "*"
