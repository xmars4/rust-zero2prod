# About
- Author Source code: https://github.com/LukeMathWalker/zero-to-production
- [Zero To Production In Rust](https://www.zero2prod.com/index.html)
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