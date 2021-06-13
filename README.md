# waveflow

waveflow is web app that handle workflow that trigerred by webhooks using hmac sha256 signature validation, initial support is github webhook,
`workflow` itself is only shell script ( for now ) maybe in the future I will support another kind of script like js or python.

waveflow built with Rust on the backend side and React on the frontend side, it's supposed to be lightweight because I need to run this project inside raspberry 3 that only has 1Gb ram.
