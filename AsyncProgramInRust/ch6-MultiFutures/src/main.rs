use futures::{StreamExt, executor::block_on, future, future::Fuse, future::FusedFuture, future::FutureExt, join, pin_mut, select, stream::{Stream,FusedStream}, try_join};

fn main() {
    block_on(count());
}

/// Exmaple of select! default and complete
async fn count(){
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            default => unreachable!(),
            complete => break,
        }
    }

    println!("result<total> is {}.",total);
}

/// Test `select_next_some`, and `Fuse::terminated()`
async fn get_new_num() -> u8 {5}
async fn run_on_new_num(_:u8) { }
async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num:u8,
){
    let run_on_new_num_fut = run_on_new_num(starting_num).fuse();
    let get_new_num_fut = Fuse::terminated();

    pin_mut!(run_on_new_num_fut,get_new_num_fut);

    loop {
        select! {
            () = interval_timer.select_next_some() => {
                // timer的时间过去之后，当get_new_num_fut结束之后
                // 重新设置一个get_new_num_fut，来获取一个new_num
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());
                }
            },
            new_num = get_new_num_fut => {
                // 初次循环，不会进入这。
                // 获取到新数值之后，重新设置`run_on_new_num_fut`
                // 然后drop旧的future
                run_on_new_num_fut.set(run_on_new_num(new_num).fuse());
            },
            () = run_on_new_num_fut => {
                // 运行run_on_new_num_fut，
            },
            // 不可能同时complete
            complete => panic!("`interval timer` completed unexpectedly."),
        }
    }
}