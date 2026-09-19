# Writing Extensions for Fasdeq Studio

Fasdeq Studio extensions are WebAssembly modules (`wasm32-unknown-unknown`)
paired with a TOML manifest. Extensions run in a sandboxed `wasmtime`
runtime with a small, explicit host API, so they can be written in any
language that compiles to WebAssembly, though the examples here use Rust.

## Directory layout

```
my-extension/
  extension.toml
  my_extension.wasm
```

Installed extensions live under `~/.config/fasdeq-studio/extensions/<id>/`.
You can install one from a local folder or by cloning a Git repository
from inside the Extensions panel.

## The manifest

```toml
id = "my-extension"
name = "My Extension"
version = "0.1.0"
author = "Your Name"
description = "What this extension does."
entry = "my_extension.wasm"

[permissions]
network = false
filesystem = false
process_spawn = false

[[contributes.commands]]
id = "myExtension.doThing"
title = "My Extension: Do Thing"

[[contributes.languages]]
id = "mylang"
extensions = ["ml"]
syntax_file = "mylang.sublime-syntax"

[[contributes.themes]]
name = "My Theme"
file = "my-theme.toml"

[[contributes.panels]]
id = "myExtension.panel"
title = "My Panel"
icon = "star"
```

Permissions are opt-in. An extension with `network = false` cannot make
outbound requests even if it calls `network_request` — the host silently
drops the call.

## Host API

Your WASM module can import the following functions from the `fasdeq`
module namespace. All string arguments are passed as a `(pointer, length)`
pair into the module's own linear memory.

- `show_message(ptr, len)` — write a line to the extension log / status bar.
- `register_command(id_ptr, id_len, title_ptr, title_len)` — expose a
  command in the command palette.
- `register_panel(id_ptr, id_len, title_ptr, title_len)` — add a panel to
  the sidebar.
- `network_request(url_ptr, url_len, method_ptr, method_len, body_ptr, body_len)`
  — make an outbound HTTP request (requires the `network` permission).
  Useful for AI assistant integrations such as calling the Claude API.
- `read_active_buffer()` — request the contents of the currently focused
  editor buffer.
- `write_active_buffer(ptr, len)` — replace the contents of the active
  editor buffer.

## Exported entry points

Your module should export:

- `activate()` — called once when the extension is loaded. Register your
  commands and panels here.
- `execute_command(ptr, len)` — called when the user runs one of your
  registered commands, with the command id passed in.

## Building

```
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/my_extension.wasm ./
```

See `examples/extensions/claude-assistant` for an AI assistant integration
and `examples/extensions/hello-language` for a minimal language
contribution.
