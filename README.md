# Tasks app
![image](https://github.com/user-attachments/assets/c6932c72-9b5d-4907-b7e7-0d491d2e9247)

I've made this repository to demonstrate the cross-platform capabilities of Tauri 2.0.
This application targets Desktop and Android platforms. 

## Required
### Tauri
Follow the steps on the Tauri prerequisites page:
https://tauri.app/start/prerequisites
### Package manager
I've used pnpm to build the Tauri application. 
You can install pnpm using (the last npm command you need...):
```bash
npm install -g pnpm@latest-10
```

## Modules

### Lab tasks srv

This module contains a Rust server. This server is added to demonstrate external communication from the Tauri application.

#### How to run
```bash
cd lab_tasks_srv
cargo run

# or if you want to watch for changes
cargo watch -x run
```

### Lab tasks Tauri

This module contains the React front end and the corresponding crates.

#### Setup
The application uses an .env file to point to the target server for external requests.
Make sure to place the .env file in the lab_tasks_tauri folder.
An example .env file contains:
```text
VITE_BASE_URL = http://localhost:8080
```

#### How to run
```bash
cd lab_tasks_tauri
pnpm I
pnpm tauri init android
# For desktop application
pnpm run tauri dev
# For Android applciation
pnpm run tauri android dev
```

#### 

### Lab tasks types

This module contains the crate of types shared by the front and back.

### Dialer plugin

The Tauri plugin focussed on demonstrating the communication from the front end to the Tauri core.

### Filepicker plugin

Tauri plugin focussed on demonstrating the communication between the backend and front end.

## How to create a Tauri Android plugin
```bash
# Initialize core plugin
npx @tauri-apps/cli plugin new [name]
# Add android plugin
pnpm run tauri plugin android init
# Add Swift :( plugin if you really want to...
pnpm run tauri plugin ios init
```
