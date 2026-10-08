# Project 5: async_chat

A multi-client TCP chat server on `tokio`.

## Skills practised
`async`/`.await`, `#[tokio::main]`, `tokio::spawn`, `broadcast` channels, `select!`, splitting a socket into read/write halves, async integration tests (`#[tokio::test]`).

**Theory:** an `async fn` returns a *future* - a state machine that does nothing until polled. An executor (tokio) polls many futures on few threads, so waiting on I/O costs no thread.

## Build it yourself
1. Echo server first: accept, read a line, write it back.
2. Spawn a task per client.
3. Add a shared `broadcast` channel so every line reaches every client.
4. Extensions: `/nick <name>`, `/who`, private messages `/msg <user> ...`, a client binary using `tokio::io::stdin`, idle timeout with `tokio::time::timeout`, graceful shutdown on Ctrl-C (`tokio::signal`).

## Run
```
cargo run            # then connect from two terminals: telnet 127.0.0.1 7878
cargo test
```
