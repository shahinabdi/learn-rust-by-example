#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:7878".into());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("chat server on {addr}  (try: telnet 127.0.0.1 7878)");
    async_chat::serve(listener).await
}
