# DevNotes

## --release (default) vs debug

1. `wasm-pack build --target web --no-typescript --out-dir www --debug`

```
ls -lh www/pkg/
Permissions Size User Date Modified Name
.rw-rw-r--   184 tac  16 Mar 13:04  package.json
.rw-rw-r--   17k tac  16 Mar 13:04  wasm_input.js
.rw-rw-r--  160k tac  16 Mar 13:04  wasm_input_bg.wasm
```

2. `wasm-pack build --target web --no-typescript --out-dir www`

```
ls -lh www/pkg/
Permissions Size User Date Modified Name
.rw-rw-r--   184 tac  16 Mar 13:09  package.json
.rw-rw-r--   15k tac  16 Mar 13:09  wasm_input.js
.rw-rw-r--   26k tac  16 Mar 13:09  wasm_input_bg.wasm
```
