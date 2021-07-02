use std::{collections::{HashMap, binary_heap::Iter}, fmt::{Debug, Display}, hash::{BuildHasher, Hash}, iter::FromIterator};

fn main() {
    
}

/// Layout
/// 
/// Total 26 bytes
/// 
/// size 26 bytes, alignment 8-byte
/// 
/// 如果向把Foo放到Vec里，那么每个item都会填充6个byte的padding=32-byte
#[repr(C)]
struct Foo{
    tiny:bool,//size 1-byte,alignment 1-byte/ byte0
    //byte1-3, 3 bytes padding
    normal:u32,//size 4-byte,alignment 4-byte/ byte4-7
    small:u8,//size 1-byte,alignment 1-byte/ byte8
    //byte9-15, 7 bytes padding
    long:u64,//size 8-byte,alignment 8-byte/ byte16-23
    short:u16,//size 2-byte,alignment 2-byte/ byte24-25
}

/// DSTs fault
// fn dsts_faults(){
//     let x:(i32,dyn Iterator,[u8],i32);// err. Sized not implemented.
// }

/// Use `Self:Sized` to limit trait objects
trait qq where Self:Sized{
    fn ss(){
    }
}
// fn ss_self_sized(s:&dyn qq){
//     //error: qq is not `object-safe`.
// }

/// impl associated type generic trait(with multiple associated types) for Point
struct Point{
    x:i32,
    y:i32,
}
trait ZZ {
    type z;
    type y;
    fn zprint(&self,other:Self::z);
}
struct QQ(i32);
struct BB(i32);
// impl ZZ for Point{
//     type z = QQ;
    
//     fn zprint(&self,other:Self::z) {
//         todo!()
//     }
// }
// will error here
// impl ZZ for Point {
//     type z = BB;

//     fn zprint(&self,other:Self::z) {
//         todo!()
//     }
// }

/// 如果我们想要构建一个返回`HashMap<K,V,S>`，其中key的类型是T，value的类型是usize
/// 的函数, 方法1 和 方法2 
fn get_hashmap_2_traitbounds<T,S>(x:HashMap<T,usize,S>)
where T:Hash+Eq,S:BuildHasher+Default
{
    let mut x = HashMap::new();
    
    x.insert("k", 123 as usize);
    let x = get_hashmap_1_traitbounds(x);
}

fn get_hashmap_1_traitbounds<T,S>(x:HashMap<T,usize,S>) -> HashMap<T,usize,S>
where HashMap<T,usize,S>:FromIterator<(T,usize)>
{
    x
}

fn get_hashmap_error_traitbounds<T,S>(x:(T,usize)) -> HashMap<T,usize,S>
where
    T:Clone+Eq+Hash,
    S:BuildHasher+Default
{
    let five_fives = std::iter::repeat(x).take(5);
    HashMap::from_iter(five_fives)
}

fn get_hashmap_correct_traitbounds<T,S>(x:(T,usize)) -> HashMap<T,usize,S>
where
    HashMap<T,usize,S>:FromIterator<(T,usize)>,
    T:Clone
{
    let five_fives = std::iter::repeat(x).take(5);
    HashMap::from_iter(five_fives)
}

/// Advanced&Advanced Trait Bounds
struct AnyIterator<T>{
    x:T,
}
impl<T> Iterator for AnyIterator<T>{
    type Item=T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
//Wrong implmentation
// impl<T> Debug for AnyIterator<T>
// where
//     Self:IntoIterator,
//     <Self as IntoIterator>::Item : Debug
// {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_list().entries(self).finish()
//     }
// }
//Correct Implmentation
impl<T> Debug for AnyIterator<T>
where
    for<'a> &'a Self:IntoIterator,
    for<'a> <&'a Self as IntoIterator>::Item : Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

/// impl IntoIterator
struct XXX{

}
// unstable usage
impl IntoIterator for XXX  
{
    type Item = i32;

    type IntoIter = impl Iterator<Item = Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![1,2,3].into_iter()
    }
}

/// impl Trait and generic types, isomorphic
fn test(){
    //you can use turbofish ::<S> in generic trait
    genreic_trait::<Vec<i32>>(vec![1,2,3]); 
    //can't use this syntax in impl Trait
    impl_traits::<Vec<i32>>(vec![1,2,3]);
}
fn impl_traits(x:impl PartialEq){

}
fn genreic_trait<T:PartialEq>(x:T){

}
// RFC 1522 usage example
fn foo(n:u32) -> impl Iterator<Item = u32>{
    (0..n).map(|x| x*100)
}
fn use_foo(){
    for x in foo(10){
        //x=0,100,200,...
    }
}

//RFC 2071#
//existential type Adder: Fn(usize) -> usize;