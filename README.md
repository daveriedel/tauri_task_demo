# Tasks app

## Lab tasks srv

This module contains a Rust server. This server is added to demonstrate external communication from the Tauri application.

### How to run
```bash
cd lab_tasks_srv
cargo run

# or if you want to watch for changes
cargo watch -x run
```

## Lab tasks Tauri

This module contains the React front end and the corresponding crates.

### How to run
```bash
cd lab_tasks_tauri
pnpm I
pnpm tauri init android
# For desktop application
pnpm run tauri dev
# For Android applciation
pnpm run tauri android dev
```

## Lab tasks types

This module contains the crate of types shared by the front and back.

## Dialer plugin

The Tauri plugin focussed on demonstrating the communication from the front end to the Tauri core.

## Filepicker plugin

Tauri plugin focussed on demonstrating the communication between the backend and front end.

### How to create a Tauri Android plugin
```bash
# Initialize core plugin
npx @tauri-apps/cli plugin new [name]
# Add android plugin
pnpm run tauri plugin android init
# Add Swift :( plugin if you really want to...
pnpm run tauri plugin ios init
```
