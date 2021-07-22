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