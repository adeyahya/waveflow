# waveflow

![waveflow](www/src/resources/logo-with-text.svg)
waveflow is web app that runs workflow trigerred by webhook using hmac sha256 signature validation, initial support is github webhook,
`workflow` itself is only shell script ( for now ) maybe in the future I will support another kind of script like js or python.

waveflow built with Rust on the backend side and React on the frontend side, it's supposed to be lightweight because I need to run this project inside raspberry 3 that only has 1Gb ram.

## How to run this project?

### Requirements

- Node.js > 12
- libsqlite3-dev
- rustup with rustc 1.52

### Installation

```bash
# this project uses yarn workspace
yarn install
cargo install diesel-cli
cp server/.env.example server/.env
cd server && diesel migration run
yarn dev
```
