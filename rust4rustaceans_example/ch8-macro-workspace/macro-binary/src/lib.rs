/// Test macro scope
mod has_macro {
    //m!{}//error here, not define
    macro_rules! m {
        () => {
            
        };
    }
    m!{}//ok here
    super::mm!();// path-based scope
    crate::mm!();// path-based scope
}
//m!{}//error here, not in scope(has_macro module)

/// Test path-based scope
mod mac {
    #[macro_export]
    macro_rules! mm {
        () => {
            
        };
    }
}
mm!();// path-based scope
self::mm!();// path-based scope，这几种调用方式都是一样的。

fn test_inner<T>(init:T,frobnify:bool){
    let xu8 = 1u8;
    println!("{}",xu8.result());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1u8_frobnified(){
        test_inner(1u8, true);
    }
    #[test]
    fn test_1i128_not_frobnified(){
        test_inner(1i128, false);
    }
}
macro_rules! test_battery {
    ($($t:ty as $name:ident),*) => {
        $(
            #[cfg(test)]
            mod $name {
                use super::*;
                #[test]
                fn frobnified(){
                    test_inner::<$t>(1,true);
                }
                #[test]
                fn unfrobnified(){
                    test_inner::<$t>(1,true);
                }
            }
        )*
    };
}
test_battery! {
    u8 as u8_test,
    i128 as i128_test
}
trait AddOne {
    type InnerType;
    fn result(&self) -> Self::InnerType;
}
macro_rules! addone_for_integer {
    ($($t:ty),*) => {
        $(
            impl AddOne for $t{
                type InnerType = $t;
                fn result(&self) -> Self::InnerType{
                    *self + 1
                }
            }
        )*
    };
}
addone_for_integer![u8,i32];
