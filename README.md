# rust-todo-app

This is a small command line todo list application written in Rust.

## Configuration

Configuration values are loaded from environment variables or a `.env` file using [`dotenvy`](https://crates.io/crates/dotenvy).

### Required variables

- `DATABASE_URL` - connection string for the database used by the application.

Create a `.env` file in the project root or export the variable in your shell:

```
DATABASE_URL=postgres://user:password@localhost/todos
```

Then run the application as usual.

## Web frontend

Running `cargo run --bin server` will start the REST server on port 8080.
The server now serves a very small HTML/JS client under `/` that can
list, add, mark and delete todo items using the same API endpoints.
