#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
#[cfg(debug_assertions)]
fn debug_code(){
    println!("Compiled while debug_assertions enbaled.");
    debug_assert!(true);
}
