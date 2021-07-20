/// # Doc Test test
/// 
/// In the first example we create a Helper instance, and exmine its x component.
/// ```
/// # use ch7::Helper;
/// # let x = Helper{
/// #   x:1
/// # };
/// assert_eq!(x.print_and_return_x("doctest"),1);
/// ```
/// 
/// In the second example we create a Helper instance, and test the `return Result` usage.
/// ```
/// # use ch7::Helper;
/// # use std::error::Error;
/// # let x = Helper{
/// #     x:"0".parse()?,
/// # };
/// assert!(x.print_and_return_err_if_1().is_ok());
/// # Ok::<(),Box<dyn Error>>(())
/// ```
/// 
/// In the Third example we create a Helper instance, and test the block replace usage.
/// 这种方式无法使用，可以看作用来隐藏注释的东西。
/// ```compile_fail
/// # /*
/// use ch7::Helper;
/// use std::error::Error;
/// let x = Helper{
///     x:"0".parse()?,
/// };
/// # */
/// assert!(x.print_and_return_err_if_1().is_ok());
/// # Ok::<(),Box<dyn Error>>(())
/// ```
pub struct Helper{
    pub x:i32,
}
/// Doc Test test
/// 
impl Helper{
    pub fn print_and_return_x(&self,s:&str)->i32{
        println!("{},{}",s,self.x);
        self.x
    }
    #[cfg(test)]
    pub fn x_plus_1(&self)->i32{
        self.x+1
    }
    pub fn print_and_return_err_if_1(&self)->Result<i32,i32>{
        if self.x == 1{
            return Err(self.x);
        }else {
            return Ok(self.x);
        }
    }
}

#[cfg(test)]
use mockall::{automock,mock,predicate::*};
#[cfg_attr(test,automock)]
trait MyTrait {
    fn foo(&self,x:u32)->u32;
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn my_test(){
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(eq(4))
            .times(1)
            .returning(|x| x+1);
        
        assert_eq!(5,mock.foo(4));
    }
    #[test]
    fn my_test_with_cfgtest(){
        let x = Helper{
            x:1,
        };
        assert_eq!(x.x_plus_1(),2);
    }
}
