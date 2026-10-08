use async_chat::serve;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::time::{timeout, Duration};

async fn read_line(r: &mut tokio::io::Lines<BufReader<tokio::net::tcp::OwnedReadHalf>>) -> String {
    timeout(Duration::from_secs(5), r.next_line()).await.expect("timed out").unwrap().expect("closed")
}

#[tokio::test]
async fn two_clients_chat() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap(); // port 0 = any free port
    let addr = listener.local_addr().unwrap();
    tokio::spawn(serve(listener));

    let (r1, mut w1) = TcpStream::connect(addr).await.unwrap().into_split();
    let mut l1 = BufReader::new(r1).lines();
    assert_eq!(read_line(&mut l1).await, "welcome user1");
    assert_eq!(read_line(&mut l1).await, "* user1 joined");

    let (r2, _w2) = TcpStream::connect(addr).await.unwrap().into_split();
    let mut l2 = BufReader::new(r2).lines();
    assert_eq!(read_line(&mut l2).await, "welcome user2");
    assert_eq!(read_line(&mut l1).await, "* user2 joined");

    w1.write_all(b"hello\n").await.unwrap();
    assert_eq!(read_line(&mut l2).await, "* user2 joined");
    assert_eq!(read_line(&mut l2).await, "user1: hello");
}
