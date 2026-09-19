# tauri-plugin-kcp

This plugin only works with Tauri 2.x only.
Based on @kuyoonjo/tauri-plugin-udp

## Install

```bash
cargo add tauri-plugin-kcp
```
```bash
npm i @jwyxym/tauri-plugin-kcp
```

## Usage

### rust
```rust

tauri::Builder::default()
    .plugin(tauri_plugin_kcp::init())
    ...
```

### javascript
```javascript
import { connect, listen, send } from "@jwyxym/tauri-plugin-kcp";

const id = 'unique-id';
await connect(id, '192.168.1.2:9090');
await send(id, 'hello');

await listen((x) => console.log(x.payload));

```

### permissions

add `"kcp:default"` into `"permissions"` list of `src-tauri\capabilities\default.json`

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  ...
  "permissions": [
    "core:default",
    ...
    "kcp:default"
  ]
}
```