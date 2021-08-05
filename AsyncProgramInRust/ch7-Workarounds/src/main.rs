fn main() {
    println!("Hello, world!");
    //require_send_call();
}

/// rust compiler's shortcomings of async `Send` analysis.
use std::rc::Rc;
#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar() {}
async fn foo(){
    {
        let x = NotSend::default();
    }
    bar().await;
}
fn require_send(_:impl Send){}

fn require_send_call(){
    require_send(foo());
}

/// Recursive in Future
use futures::{FutureExt, future::BoxFuture};
fn recursive() -> BoxFuture<'static,()>{
    async move {
        recursive().await;
        recursive().await;
    }.boxed()
}