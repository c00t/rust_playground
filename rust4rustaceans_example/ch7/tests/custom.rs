use ch7;

fn main(){
    let x = ch7::Helper{
        x:1,
    };
    // make sure failed to print out the println! log.
    assert_eq!(x.print_and_return_x("main"),1);

    // Test code run time
    let mut vs = Vec::with_capacity(4);
    let start = std::time::Instant::now();
    for i in 0..4 {
        vs.push(i);
    }
    println!("took {:?}",start.elapsed());
}

#[test]
fn test_helper_eq(){
    let x = ch7::Helper{
        x:1,
    };
    assert_eq!(x.print_and_return_x("test"),2);
}