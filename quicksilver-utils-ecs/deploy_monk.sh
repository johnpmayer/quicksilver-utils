#! /usr/bin/env bash

cargo web deploy --release --features stdweb
cp -R ../target/deploy/* monk-deploy/
(cd monk-deploy/ && git checkout index.html) # I have audio too