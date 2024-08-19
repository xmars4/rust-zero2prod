# About

## Rust Toolchain

- IDE
    - Rust-analyzer
    - RustRover IntelliJ
- Inner Development Loop
  ```shell
  cargo install cargo-watch
  cargo watch -x check -x test -x run
  ```
- CI
    - Code coverage
      ```shell
        cargo install cargo-tarpaulin
        cargo tarpaulin --ignore-tests
      ```
    - Linting
      ```shell
          cargon clippy
      ```
    - Formating
      ```shell
          cargo fmt
      ```
    - Security Vulnerabilities
      ```shell
           cargo install cargo-audit
           cargo audi
      ```