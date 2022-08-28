Tried to make minimal example but unsure it will work with other computers.

- Kakoune v2021.11.08
- kak-lsp 12.1.0-snapshot
- Rust rls 1.41.0 (rustc/cargo 1.63.0)
- on Alacritty 0.10.1
- on Sway 1.7
- on Arch linux x64.

## How to reproduce
1. Clone this, and apply kakrc: `KAKOUNE_CONFIG_DIR="$(realpath .)" kak main.rs`
2. Select `let ir`, enter insert mode, type `: Iss`, and `<c-n>`.

### More fun things
```sh
# this one can reproduce:
kak -n -E 'source /usr/share/kak/autoload/filetype/rust.kak; source ./kakrc' main.rs

# while this one cannot:
kak -n -E 'source ./kakrc' main.rs
```
