# certbun
Cli for downloading SSL certificate bundle from Porkbun

## Usage
```
certbun <DOMAIN> <API_KEY> <SECRET_KEY>
```

## Requirements
- Nightly Rust
  ```
  rustup default nightly
  ```
- On Raspbian you'll probsbly need to download libssl-dev
  ```
  sudo apt install libssl-dev
  ```

## Installation
```
cargo install --git https://github.com/vetleras/certbun
```
