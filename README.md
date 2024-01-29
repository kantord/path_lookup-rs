# path_lookup-rs
Find all available executable commands in ``$PATH`` in Rust.


## Install

Add `path_lookup` to your `Cargo.toml`:

```bash
cargo add path_lookup
```

## Usage

`get_executables()` returns a `HashSet<String>`:

```rust
let all_commands: HashSet<String> = get_executables();
```
