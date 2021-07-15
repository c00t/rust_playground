fn main() {
    //println!("Hello, world!");

    // test cfg! macro
    if cfg!(feature = "derive") {
        println!("feature derive enabled!");
    }
}
#[cfg(feature = "derive")]
struct Helper{
    x:i32,
    y:i32,
}