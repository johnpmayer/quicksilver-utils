# Example Quicksilver Project

Quicksilver supports 3 backends. This fully-functional
example project demonstrates all three targets.

## Features

* Exit the program on 'ESC' key (on web this doesn't close the tab, just the event loop)
* Log a tick twice a second
* Log all window events
* Trigger an async "ping" task on pressing the 'P' key

## Desktop

```sh
# Builds the project and runs it as a native desktop application
cargo run
```

## Web: std-web via cargo web

```sh
# Builds the project and runs it on a local webserver. Auto-reloads when the project changes
cargo web start --features stdweb --bin index
```

## Web: web-sys via npm & wasm-pack

Note the additional flags passed to WasmPackPlugin in webpack.config.js

```sh
# Fetches additional toolchain and dependencies
npm install
```

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```
