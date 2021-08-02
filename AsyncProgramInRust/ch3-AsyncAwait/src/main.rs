fn main() {
    //println!("Hello, world!");
    futures::executor::block_on(async_blocks());
    futures::executor::block_on(async_move_blocks());
}

/// Test `async fn` and `async blocks`
/// async fn
// `foo()` 返回一个实现了`Future<Output = u8>`的类型
// 当使用了`foo().await`的时候，最终会返回一个u8类型的值
async fn foo() -> u8 {
    5
}
use std::{future::Future, process::Output};
/// async blocks
fn bar() -> impl Future<Output = u8>{
    // async blocks
    async {
        5
    }
}
use futures;
/// async blocks without move
async fn async_blocks() {
    let my_string = "my_string1".to_string();
    let future_one = async {
        println!("{}",my_string);
    };
    let future_two = async {
        println!("{}",my_string);
    };
    futures::join!(future_one,future_two);
}
fn async_move_blocks() -> impl Future<Output = ()>{
    let my_string = "my_string2".to_string();
    let block = async move {
        println!("{}",my_string);
    };
    //println!("{}",my_string);// error: value moved.
    block
}