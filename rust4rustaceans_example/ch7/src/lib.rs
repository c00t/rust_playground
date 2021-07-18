/// Doc Test test
/// 
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
