use std::{future::Future, string, sync::mpsc::Receiver, sync::mpsc::Sender};

/// 这种方式不允许嵌套Scope
macro_rules! pin_mut {
    ($var:ident) => {
        let mut $var = $var;
        let mut $var = unsafe{
            ::std::pin::Pin::new_unchecked(&mut $var)
        };
    };
}

/// 这种方式允许嵌套Scope
macro_rules! pin_mut_self {
    ($var:ident) => {
        let mut $var = unsafe{
            ::std::pin::Pin::new_unchecked(&mut $var)
        };
    };
}

fn main() {
    println!("Hello, world!");
    
    let mut x = String::from("123");
    pin_mut!(x);
    println!("{}",x);

    let mut x = String::from("123");
    {
        pin_mut_self!(x);
        //methods that take `Pin<&mut Self>`
        println!("{}",x);//`Pin<&mut String>`
    }
    println!("{}",x);//这里的类型为String
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

// impl Future for xxx{
//     type Output;

//     fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         todo!()
//     }
// }

