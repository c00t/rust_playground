use std::io;
use std::pin::Pin;

use futures::SinkExt;
use futures::Stream;
use futures::StreamExt;
use futures::channel;
fn main() {
    //println!("Hello, world!");
    futures::executor::block_on(send_recv());

    let a = ();// unit value也是可以直接赋值的
}
async fn send_recv(){
    const BUFFER_SIZE:usize = 10;
    let (mut tx, mut rx) = channel::mpsc::channel(BUFFER_SIZE);

    tx.send(1).await;
    //需要注意的是，因为`send`现在返回的是`Future`，需要有东西来consume它才行。比如.await.
    tx.send(2).await;
    drop(tx);

    assert_eq!(Some(1),rx.next().await);
    assert_eq!(Some(2),rx.next().await);
    assert_eq!(None,rx.next().await);
}

/// Iterate over Stream
async fn sum_with_next(mut stream:Pin<&mut dyn Stream<Item = i32>>) -> i32{
    let mut sum = 0;
    while let Some(item) = stream.next().await {//use next
        sum += item;
    }
    sum
}
async fn sum_with_try_next(mut stream:Pin<&mut dyn Stream<Item = Result<i32,io::Error>>>) -> 
    Result<i32,io::Error>
{
    use futures::TryStreamExt;
    let mut sum = 0;
    while let Some(item) = stream.try_next().await? {//use try_next
        sum += item;
    }
    Ok(sum)
}