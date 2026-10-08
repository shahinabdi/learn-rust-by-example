//! Tiny TCP chat server. Each client gets a task; a `broadcast` channel fans
//! every line out to everybody.

use std::io;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

pub async fn serve(listener: TcpListener) -> io::Result<()> {
    let (tx, _) = broadcast::channel::<String>(64);
    let mut next_id = 1u32;
    loop {
        let (socket, _) = listener.accept().await?;
        let tx = tx.clone();
        let name = format!("user{next_id}");
        next_id += 1;
        // spawn: each connection runs concurrently; `async move` takes ownership of what it needs
        tokio::spawn(async move {
            let _ = handle_client(socket, name, tx).await;
        });
    }
}

async fn handle_client(socket: TcpStream, name: String, tx: broadcast::Sender<String>) -> io::Result<()> {
    let (read_half, mut write_half) = socket.into_split();
    let mut rx = tx.subscribe();
    let mut lines = BufReader::new(read_half).lines();
    write_half.write_all(format!("welcome {name}\n").as_bytes()).await?;
    let _ = tx.send(format!("* {name} joined"));

    loop {
        // select!-like behaviour: whichever happens first wins
        tokio::select! {
            line = lines.next_line() => match line? {
                Some(text) => { let _ = tx.send(format!("{name}: {text}")); }
                None => break,
            },
            msg = rx.recv() => match msg {
                Ok(m) => write_half.write_all(format!("{m}\n").as_bytes()).await?,
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(_) => break,
            },
        }
    }
    let _ = tx.send(format!("* {name} left"));
    Ok(())
}
