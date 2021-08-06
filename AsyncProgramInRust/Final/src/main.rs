use async_std::net::{TcpStream, TcpListener};
// use std::io::{Read, Write};
use async_std::task;
use std::time::Duration;
use std::fs;
use futures::stream::Stream;
use futures::{AsyncReadExt, AsyncWriteExt, StreamExt};
use async_std::task::spawn;
use async_std::io::{Read,Write};
use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::cmp::min;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7876").await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(None,|tcpstream| -> _ {
            async move {
                let stream = tcpstream.unwrap();
                spawn(handle_connection(stream));
            }
        }).await;

}

async fn handle_connection(mut stream:impl Read + Write + Unpin){
    // return type is `Future<Output = ()>`
    // mut needed? ownership taken here?

    let mut buffer = [0;1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line,filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK\r\n\r\n","hello.html")
    }else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n","404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}",status_line,contents);

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

/// Unit Test for `handle_connection`
struct MockTcpStream{
    read_data:Vec<u8>,//used to store read/write data
    write_data:Vec<u8>,
}
impl Read for MockTcpStream{
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>> {
        let size:usize = min(self.read_data.len(),buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        Poll::Ready(Ok(size))
    }
}
impl Write for MockTcpStream{
    fn poll_write(mut self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<std::io::Result<usize>> {
        self.write_data = Vec::from(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}
impl Unpin for MockTcpStream{}

#[async_std::test]
async fn test_handle_connection(){
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8;1024];
    contents[..input_bytes.len()].copy_from_slice(input_bytes);
    let mut stream = MockTcpStream{
        read_data: contents,
        write_data: Vec::new(),
    };
    handle_connection(&mut stream).await;
    let mut buf = [0u8;1024];
    //stream.read(&mut buf).await;

    let expected_contents = fs::read_to_string("hello.html").unwrap();
    let expected_response = format!("{}{}","HTTP/1.1 200 OK\r\n\r\n",expected_contents);
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}