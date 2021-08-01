use std::process::Output;
use std::thread;
use futures;
use futures::executor::block_on;
fn main() {
    let fut = hello_world();
    block_on(fut);

    block_on(async_main());
}

struct Song;
async fn learn_song()->Song{println!("learn a song.");Song{}}
async fn sing_song(song:Song){println!("sing a song.");}
async fn dance(){println!("dance.");}

async fn learn_and_sing(){
    let song = learn_song().await;
    sing_song(song).await;
}
async fn async_main(){
    let fut1 = learn_and_sing();
    let fut2 = dance();

    futures::join!(fut1,fut2);
}


async fn hello_world(){
    println!("hello, world!");
}

fn get_two_websites_by_threads(){
    //spawn two threads to do work.
    let thread_one  = thread::spawn(|| download_sync("www.baidu.com"));
    let thread_two = thread::spawn(|| download_sync("www.google.com"));

    // join to wait completing
    thread_one.join().expect("thread one err.");
    thread_two.join().expect("thread two err.");
}
fn download_sync(url:&str){

}

async fn get_two_websites_by_async(){
    // create two futures,
    let future_one = download_async("www.baidu.com");
    let future_two = download_async("www.google.com");
    // join two futures, waiting all of two to complete.
    futures::join!(future_one,future_two);
    // 这里是创建了一个executor么？还是就是简单的loop
    // 看了源代码：
    // 
}
async fn download_async(url:&str){

}

enum Poll<T>{
    Ready(T),
    Pending,
}
trait SimpleFuture{
    type Output;
    fn poll(&mut self,wake:fn())->Poll<Self::Output>;
}
struct JoinFuts<FutureA,FutureB>{
    // 假定在创建时已经存在这两个Future，并且在执行结束后，两个Option都会被清空为None
    a:Option<FutureA>,
    b:Option<FutureB>,
}
impl<FutureA,FutureB> SimpleFuture for JoinFuts<FutureA,FutureB>
where
    FutureA:SimpleFuture<Output = ()>,
    FutureB:SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self,wake:fn())->Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) { //那这样每次调用poll都会更新wake函数指针啊？
                self.a.take();
            }
        }

        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take();
            }
        }

        if self.a.is_none() & self.b.is_none() {
            Poll::Ready(())
        }else{
            Poll::Pending
        }
    }
}

// 同样认为在创建AndThenFut时，这两个Future都已经存在了。
// 而，在futures库中的实现，
// `AndThen` combinator allows creating the second future based on the output
// of the first future, like `get_breakfast.and_then(|food| eat(food))`.
struct AndThenFut<FutureA,FutureB>{
    a:Option<FutureA>,
    b:FutureB,
}
impl<FutureA,FutureB> SimpleFuture for AndThenFut<FutureA,FutureB>
where
    FutureA:SimpleFuture<Output = ()>,
    FutureB:SimpleFuture<Output = ()>,
{
    type Output = ();

    fn poll(&mut self,wake:fn())->Poll<Self::Output> {
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take();
            }else{
                return Poll::Pending;
            }
        }
        self.b.poll(wake)
    }
}
