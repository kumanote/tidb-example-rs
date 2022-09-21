# tidb-example-rs

## Outline

It's an example for Rust and TiDB. Contains subproject:

- [diesel example](#diesel-example)
- [http example](#http-example)

This is a process about the game, each player has two attributes,
`coins` and `goods`, and each player has their own unique `id` as an identifier.
Players can trade with each other, provided that the `coins` and `goods` are sufficient

The process is as follows:

1. Create a player
2. Create some players
3. Read players amount
4. Read some players attributes
5. Two player trade with insufficient coins or goods
6. Two player trade with sufficient coins or goods

## Dependency

- [Rust with Cargo](http://rust-lang.org)
  - There is no specific `MSRV(Minimum Supported Rust Version)`
  - Only tested with the latest stable version Rust compiler (older/nightly builds may work...)
- [diesel_cli](https://crates.io/crates/diesel_cli)

## Diesel example

It's an example used [diesel](https://diesel.rs/) with `mysql` feature to connect TiDB.

### Running way

- Run `diesel migration run` to init schema in your tidb.
- Run `cargo build --release` to build entrypoint to test.
- Run `./target/release/tidb-example-cmd` to start.

### Expected output

diesel example [expected output](./Expected-Output.md#Diesel)

### Code

- [Main Entry](./cmd/src/main.rs)

## http example

It's an example service used diesel to connect TiDB.
Provide a group of HTTP Restful interface.

### Running way

- Run `diesel migration run` to init schema in your tidb.
- Run `cargo build --release` to build entrypoint to test.
- Run `./target/release/tidb-example-server` to start.
- Using [script](./http/request.sh) to request. It's based on `curl`

### Expected output

1. request [expected output](./Expected-Output.md#http-request)

### Code

- [Main Entry](./http/src/main.rs)


## License

This software includes the work that is distributed in the Apache License 2.0.

- [tidb-example-golang/LICENSE](https://github.com/pingcap-inc/tidb-example-golang/blob/main/LICENSE)
