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

## Formatting

To format the code, you can use `cargo fmt` to format rust code and `dioxus fmt` to format rsx code.
They might sometimes be in conflict, in that case `rustfmt` is right.

## Vim config

`rustfmt` should run automatically if rust-analyzer is setup correctly.
In addition, you can enter this command to format rsx on save :

```vim
autocmd BufWritePost *.rs silent !dioxus fmt
```
