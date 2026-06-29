# Rusty

A collection of Rust projects organized by skill level

## Running

Run a specific project by name:

```sh
cargo run -p hello
```

Or from within a project directory:

```sh
cd beginner/hello && cargo run
```

## Adding a new project

1. Create the crate:
```sh
cargo init beginner/foo
```

2. Register it in `Cargo.toml`:
```toml
members = ["beginner/hello", "beginner/foo"]
```

3. Run it:
```sh
cargo run -p foo
```
