use std::process::Output;
use std::sync::{Arc, Mutex};
use std::task::Waker;
use std::thread;
use std::future;
use std::time::Duration;
use futures;
use futures::executor::block_on;
use std::task::Poll;
use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        task::Context,
    },
};
fn main() {
    let fut = hello_world();
    block_on(fut);

    block_on(async_main());

    // Test Executor and TimerFuture
    let (executor,spawner) = new_executor_and_spawner();
    // 可以先传到channel里面去，因为里面是有inner buffer的
    spawner.spawn(async {
        println!("howdy!");
        // wait for TimerFuture to complete.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("Wait end!\nDone!");
    });

    // 一旦Spawner被drop之后，executor就知道自己绝对不会收到新的tasks了
    // 这个代码没有也行吧？可以试试看,没有的话会导致程序不会退出
    // 那其实还是drop之后会导致Sender消失啊
    drop(spawner);

    executor.run();

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

// enum Poll<T>{
//     Ready(T),
//     Pending,
// }
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

/// Build a Timer
struct TimerFuture{
    shared_state:Arc<Mutex<SharedState>>,
}
/// Future和Waiting Thread之间共享的数据
struct SharedState{
    /// 用来提示sleep time是否过去了
    complete:bool,
    /// 属于`TimerFuture`的waker,
    /// 在waiting thread计算完时间之后，把`complete`设置为true，然后利用这个waker来提示executor,wakeup这个Future
    waker:Option<Waker>,
}
/// 给TimerFuture实现Future这个Trait
impl future::Future for TimerFuture{
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        // 首先看看shared_state，timer是否已经完成了
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.complete {
            Poll::Ready(())
        }else{
            // 设置waker，以便waiting thread在时间过去之后能够唤醒当前task，并让future确认complete = true。
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
// 创建waiting thread
impl TimerFuture{
    fn new(duration:Duration) -> Self{
        let shared_state = Arc::new(Mutex::new(
            SharedState {
                complete:false,
                waker:None,
            }
        ));
        // 生成一个线程来计时，并且给shared_state增加一个计数
        // Arc有Arc::clone和直接clone的区别吗？
        let thread_shared_state = Arc::clone(&shared_state);
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 提示timer已经结束了，并且wake相关的task
            shared_state.complete = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });
        TimerFuture{shared_state}
    }
}

/// Build a Executor
struct Executor{
    /// channel for receive tasks
    ready_queue:Receiver<Arc<Task>>,
}
/// 给Executor实现从ready_queue中获取task，并且给poll的方法
impl Executor{
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            println!("out: {}",Arc::strong_count(&task));
            // 把task中的future拿出来，如果其还未完成，（仍为Some）
            // 就Poll它
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                println!("in1: {}",Arc::strong_count(&task));
                // 为了poll，首先需要创建一个waker
                // 这里waker对task对应的东西还是有指针的，导致task无法drop
                let waker = waker_ref(&task);
                // 从waker中创建一个context
                let context = &mut Context::from_waker(&waker);
                println!("in2: {}",Arc::strong_count(&task));
                if let Poll::Pending = future.as_mut().poll(context) {
                    // 当尚未结束的时候，需要把future再存回去
                    *future_slot = Some(future);
                }
                // 当已经结束了，future就被直接释放掉了
                println!("in3: {}",Arc::strong_count(&task)); // string count: from 2 to 1
            }
        }
    }
}

/// `Spawner`生成新的future，并且发送到task channel里
#[derive(Clone)]
struct Spawner {
    // SyncSender 即同步channel，如果内部buffer不足时，该type的send函数会block
    task_sender:SyncSender<Arc<Task>>,
}
/// `Task` is a future that can reschedule itself
struct Task {
    /// The `Mutex` is not necessary for correctness, since we only have
    /// one thread executing tasks at once. However, Rust isn't smart
    /// enough to know that `future` is only mutated from one thread,
    /// so we need to use the `Mutex` to prove thread-safety. A production
    /// executor would not need this, and could use `UnsafeCell` instead.
    future:Mutex<Option<BoxFuture<'static,()>>>,//BoxFuture 是 Box<Future + Send> 的别名

    // channel handle to send tasks back to executor
    task_sender:SyncSender<Arc<Task>>,
}
/// Create executor and spawner
fn new_executor_and_spawner()->(Executor,Spawner){
    // 最大允许push queue进channel的task数量
    const MAX_QUEUED_TASKS:usize = 10_000;
    let (task_sender,ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor{ready_queue},Spawner{task_sender})
}
/// 给Spawner添加一个方法以便其spawn futures。
/// his method will take a future type, box it,
/// and create a new Arc<Task> with it inside 
/// which can be enqueued onto the executor.
impl Spawner{
    fn spawn(&self,future:impl Future<Output = ()> + 'static + Send){
        let future = future.boxed();
        // 这里相当于是创建了一个task，然后得到了一个Arc指针
        let task = Arc::new(Task{
            future:Mutex::new(Some(future)),
            task_sender:self.task_sender.clone(),
        });
        // 然后又把这个指针传递给Executor了
        self.task_sender.send(task).expect("too many tasks queued.");
    }
}
/// 为了poll一个future，我们必须要创建一个waker，
impl ArcWake for Task{
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 通过将Task发送到channel端来实现wake
        // println!("{}",Arc::strong_count(arc_self)); // strong count 1
        let cloned = arc_self.clone();
        // println!("{}",Arc::strong_count(arc_self)); // strong count 2
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued.");
    }
}