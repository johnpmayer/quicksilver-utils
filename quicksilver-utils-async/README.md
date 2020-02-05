
# Quicksilver Async Utilities

Components primarily designed to be used with the Quicksilver game
engine, though quicksilver is not a dependency and these could
conceievably be useful in any cross-platform async project.

Mostly this implies two things about each component

1. Runs on desktop and on the web (both cargo-web and wasm-bindgen)
2. Async api

## Current

* Cooperative tasks with an event buffer
* Async sleep() function
* Async Websocket Client
* Async HTTP Client

## Planned

* More sophistocated timers (look at async-timer)
