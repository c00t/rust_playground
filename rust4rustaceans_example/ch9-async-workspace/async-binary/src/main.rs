use std::{sync::mpsc::Receiver, sync::mpsc::Sender};

fn main() {
    println!("Hello, world!");
}

/// Manually implementing a channel-forwarding future
/// implement below psuse code
/// ```no_run
/// async fn forward<T>(rx: Receiver<T>, tx: Sender<T>) {
///     while let Some(t) = rx.next().await {
///     tx.send(t).await;
///     }
/// }
/// ```
struct PlaceHolder;
// enum Forward<T> {
//     WaitingForReceive(ReceiveFuture<T>, Option<Sender<T>>),
//     WaitingForSend(SendFuture<T>, Option<Receiver<T>>),
// }

// impl<T> Future for Forward<T>{
//     type Output = ();

//     fn poll(&mut self) -> Poll<Self::Output> {
//         match self {
//             Forward::WaitingForReceive(recv,tx) => {
//                 if let Poll::Ready((rx,v)) = recv.poll() {
//                     if let Some(v) = v{
//                         let tx = tx.take().unwrap();
//                         *self = Forward::WaitingForSend(tx.send(v),Some(rx));
//                         // Try to make progress on sending
//                         return self.poll();
//                     }else{
//                         Poll::Ready(())
//                     }
//                 }else{
//                     Poll::Pending
//                 }
//             },
//             Forward::WaitingForSend(send,rx) => {
//                 if let Poll::Ready(tx) = send.poll() {
//                     let rx = rx.take().unwrap();
//                     *self = Forward::WaitingForSend(rx.receive(),Some(tx));
//                     // Try to make progress on receiveing
//                     return self.poll();
//                 }else{
//                     Poll::Pending
//                 }
//             },
//         }
//     }
// }
