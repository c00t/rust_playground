#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct ThreadPool{

}
impl ThreadPool{
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool. 
    /// 
    /// # Example
    /// ```
    /// let pool = ThreadPool::new(3);
    /// ```
    ///
    /// # Panics
    /// 
    /// The `new` function will panic if the size is 0.
    pub fn new(pool_size:usize)->ThreadPool{// usize here 主要是为了方便索引，不用再进行类型转换了
        assert!(pool_size>0);
        
        ThreadPool{

        }
    }
    pub fn execute<T>(&self , closure:T)
    where
        T:FnOnce() + Send + 'static
    {
        
    }
}
