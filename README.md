
# vimcord

`vimcord` is a lightweight **Discord Rich Presence daemon** designed to integrate **Vim / Neovim running inside WSL** with the **Discord client running on Windows**.

It aims to address a common limitation where editors running in WSL cannot directly communicate with Discord Rich Presence due to IPC and OS boundaries.

---

## Features

- Discord Rich Presence for **Vim / Neovim on WSL**
- Rust-based **daemon + CLI**
- Simple HTTP bridge between WSL and Windows
- Neovim plugin (Lua) for automatic updates
- Low-overhead and minimal design
- No editor lock-in (CLI can be used by any editor)

---

## How it works
```bash
Neovim (WSL)
     ↓
vimcord CLI (WSL)
     ↓ HTTP
vimcord daemon (Windows)
     ↓ IPC
Discord Client (Windows)
```
- The daemon runs on **Windows**
- Editors and plugins run inside **WSL**
- Communication happens via a small local HTTP server
- Discord Rich Presence is handled on the Windows side

---

## Components

- **Daemon** (Rust, Windows)  
  Maintains the Discord RPC connection and syncs presence updates.
- **CLI** (Rust, WSL)  
  Sends presence updates to the daemon.
- **Neovim Plugin**  
  Automatically updates presence on editor events.

---

## Installation

### 1. Clone the repository
```bash
git clone https://github.com/AghastyGD/vimcord.git
cd vimcord
```

### 2. Set the Discord Client ID
Create a `.env` file on Windows:

```bash
DISCORD_CLIENT_ID=your_discord_app_id
```

You can create an application at: https://discord.com/developers/applications

### 3. Run the daemon (Windows)
```bash
cargo run -- daemon
```
Expected output:
```bash
vimcord daemon running on :8787
```
### 4. Install the CLI (WSL)
```bash
cargo install --path .
```
Verify:
```bash
vimcord --help
```
---

## Usage

### Update presence manually
```bash
vimcord update --details "In Vim" --state "Editing" --file main.rs
```
### Clear presence
```bash
vimcord clear
```
---

## Neovim Plugin

The repository includes a simple Neovim plugin.

Example using **lazy.nvim**:
```lua
{
  dir = "~/path/to/vimcord/plugins/vimcord.nvim",
  lazy = false,
  config = function()
    require("vimcord").setup()
  end
}
```

The plugin automatically updates Discord presence when entering or saving files.

---


## Known Limitations

- Discord Rich Presence may be cleared after ~30 seconds in certain setups, even while the daemon is running.
- This behavior appears to be influenced by:
  - Discord client version
  - Other active Rich Presence integrations
  - Use of external (non-embedded) RPC clients

**Mitigation:**  
To improve stability, `vimcord` uses a low-frequency heartbeat that re-sends the current activity without modifying timestamps.

---

## Roadmap

 - [ ] File language detection
-  [ ] Discord assets (icons)
-  [ ] Idle detection
-  [ ] Config file support
-  [ ] Plugin improvements
