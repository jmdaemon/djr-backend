# README

## Usage

To run the REST API service:

```bash
cargo run --bin djr-backend
```

## Connecting

To connect to the database directly as root, run:

```bash
mysql -h 127.0.0.1 -P 5432 -u root -p djr
```

## Hot Reloading

To enable hot-reloading the REST API whenever files in `src` are modified, run:

```bash
cargo-watch -q -c -w src/ -x 'run --bin djr-backend'
```

You will need to install `cargo-watch` in order to enable hot-reloading:

```bash
cargo install cargo-watch
```

## Testing

For testing the REST API, we mock interactions
using pre-supplied test fixtures and data.

For testing the diesel ORM integration, we mainly use a script
`test-endpoints.sh` to test the live endpoints via `curl`.
