# Websocket Chat Server

This is a suggested solution of the assignments to write a HTTP chat server using [`actix-web`](https://github.com/actix/actix-web) and `websockets` with [`actix-web-actors`](https://github.com/actix/actix-web/tree/master/actix-web-actors).

The assignment can be found here: https://ferrous-systems.github.io/teaching-material/target/assignments/async-mailbox.html

To start the chat server execute `cargo run`. To connect to the chat server via client, open your browser at http://localhost:8080/.

To enable logs set the `RUST_LOG` environment variable.

```shell
RUST_LOG=info cargo run
```

## Adjustments
