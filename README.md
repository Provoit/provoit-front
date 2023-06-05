# Provoit front

Provoit frontend

## Running

### Prerequisites

First of all, install the wasm target.

```sh
rustup target add wasm32-unknown-unknown
```

Then you'll need `dioxus-cli` installed :

```sh
cargo install dioxus-cli
```

### Web

To serve the project on the web, use the following command

```sh
dioxus serve
```

### Desktop

To run on desktop, use

```sh
dioxus serve --platform desktop
```

or

```sh
cargo run
```
