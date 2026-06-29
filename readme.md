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

Create a crate (e.g., `beginner/foo`), then add it to `members` in `Cargo.toml`:

```toml
members = ["beginner/hello", "beginner/foo"]
```
