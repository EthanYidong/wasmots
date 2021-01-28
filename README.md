# WASM on the side

Toy project. Don't use. If you do, bad things will happen.

## What is this?

In the past, crowdsourcing CPU power has been done through Docker. Docker was nice because containers a) run everywhere, and b) can't brick your system / steal your credit cards / delete system32. However, restrictions include: a) docker runtime is thick, b) containers are overkill for most cases, and c) they are CPU architecture-dependant.

WASM can also run anywhere, but it's lighter than docker. As WASI matures and gains more features, things like network access will also be possible.

## Example

To run example: 

`cargo build -p example-wasm --target wasm32-wasi --release` 

to build the WASM file, then

`cargo run -p example-server`

In another window, run

`cargo run -p wasmots-client -- http://localhost:3030`
