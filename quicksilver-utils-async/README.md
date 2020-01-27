
# Quicksilver Async Utilities

Components primarily designed to be used with the Quicksilver game
engine, though quicksilver is not a dependency and these could
conceievably be useful in any cross-platform async project.

Mostly this implies two things about each component

1. Runs on desktop and on the web (primarily targeting stdweb for now)
2. Async api

## Current

* Cooperative tasks with an event buffer
* Blocking sleep() function

## Planned

* Blocking Websocket Client
* Blocking HTTP Client
* More sophistocated timers
