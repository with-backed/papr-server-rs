`cargo run --bin write_twabs`

# Database stuff

## Requirements
- diesel cli
  - https://diesel.rs/guides/getting-started.html
  - install `libpq` first
  - `cargo install diesel_cli --no-default-features --features postgres`
- docker
  - `docker-compose up -d postgres`

To inspect the DB:
`docker exec -it <container name> psql -U user -W papr`
And then you can just use psql commands or type in sql queries.