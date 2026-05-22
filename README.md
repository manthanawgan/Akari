# Akari

Akari is a tiny sticky-note todo popup for Arch Linux, Hyprland, and Waybar.

It stores todos in `~/.local/share/akari/todos.json`, shows the incomplete count in Waybar, and opens as a small frameless Tauri window.

## Features

- Add a todo with Enter
- Mark todos done with a checkbox
- Delete todos with the hover `x` button
- Persistent JSON storage
- Waybar badge for incomplete todos
- Waybar click toggles the popup through Tauri single-instance behavior

## Requirements

- Rust and Cargo
- Node.js and npm
- Tauri 2 Linux prerequisites
- Hyprland
- Waybar

On Arch Linux, install the common Tauri desktop dependencies:

```bash
sudo pacman -S --needed webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module librsvg
```

## Develop

```bash
npm install
npm run tauri:dev
```

## Build

```bash
npm install
npm run tauri:build
```

After building, install or symlink the generated `akari` binary somewhere in your `PATH` so Waybar can launch it:

```bash
sudo install -Dm755 src-tauri/target/release/akari /usr/local/bin/akari
```

## Waybar

Install the Waybar script:

```bash
mkdir -p ~/.config/waybar/scripts
install -Dm755 waybar/akari.sh ~/.config/waybar/scripts/akari.sh
```

Add this module to your Waybar `config.json`:

```json
{
  "custom/akari": {
    "exec": "~/.config/waybar/scripts/akari.sh",
    "interval": 5,
    "on-click": "akari",
    "return-type": "",
    "format": "{}"
  }
}
```

Then add `"custom/akari"` to `modules-right`, `modules-left`, or `modules-center`.

## Hyprland

Add these rules to your Hyprland config:

```conf
windowrule = match:class ^(akari)$, float on, size 380 500, move 100%-390 30
```

Adjust the `move` rule if your Waybar is not at the top.

## Storage

Todos live at:

```text
~/.local/share/akari/todos.json
```

The JSON shape is:

```json
[
  {
    "id": "uuid-string",
    "text": "Buy groceries",
    "done": false,
    "created_at": "2026-05-23T03:30:00Z"
  }
]
```

Akari also writes `~/.local/share/akari/last_modified` on every change.
