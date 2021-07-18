pub struct Helper{
    pub x:i32,
}
impl Helper{
    pub fn print_and_return_x(&self,s:&str)->i32{
        println!("{},{}",s,self.x);
        self.x
    }
}