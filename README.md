# path_lookup-rs
Find all available executable commands in ``$PATH`` in Rust.


## Install

Add `path_lookup` to your `Cargo.toml`:

```bash
cargo add path_lookup
```

## Usage

`iterate_executables()` returns an `Iterator<Item = String>`:

```rust
for command in iterate_executables() {
    // ... etc ...
}
``

`get_executables()` returns a `HashSet<String>`:

```rust
let all_commands: HashSet<String> = get_executables();
```
