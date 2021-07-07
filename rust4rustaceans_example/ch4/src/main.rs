fn main() {
    println!("{}",frobnicate1(String::from("1")));
    println!("{}",frobnicate2("2"));// use Cow's Deref feature
    let x = String::from("3");
    println!("{}",frobnicate3(x).as_ref());

    args_types_choice();
}

/// three types of functions
fn frobnicate1(s: String) -> String{
    s + "-frobnicate1"
}
// require caller own the data:String，return a owned data:String.
use std::borrow::Cow;
fn frobnicate2(s: &str) -> Cow<'_, str>{
    Cow::from(String::from(s) + "-frobnicate2")
}
// Cow是啥？
// 使用用户不需要放弃分配内存，或者放弃传入数据的Ownership
// 返回的类型为`std::borrow::Cow`，这意味着其可以返回一个String引用或是owned String
fn frobnicate3(s: impl AsRef<str>) -> impl AsRef<str>{
    String::from(s.as_ref()) + "-frobnicate3"
}
// 第三种方法把上面两种方法的限制都去掉了，
// 它只需要某种能产生对String的引用的类型，返回值的类型也类似。

/// dynamic&static dispatch choice
trait ChoiceDispatch {
    fn ss(&self){
        println!("123");
    }
}
struct Choice{
}
impl ChoiceDispatch for Choice {
}
fn args_generic(t:impl ChoiceDispatch){
    t.ss();
}
fn args_trait_objects(t:&dyn ChoiceDispatch){
    t.ss();
}
fn args_types_choice(){
    let x_generic = Choice{};
    args_generic(x_generic);
    let x_generic = Choice{};
    let box_x_generic = Box::new(x_generic);
    args_generic(*box_x_generic);//auto deref to Choice

    let x_generic = Choice{};
    args_trait_objects(&x_generic);
}

/// use where Self:Sized to constraint things
/// function `ss` can't be invoke on trait object.
trait ConsInstance {
    fn ss()
    where Self:Sized
    {
        println!("123");
    }
}
struct ConsStruct{}
impl ConsInstance for ConsStruct{
}
fn test_where_self_sized(){
    fn cal_without_instance(t:&dyn ConsInstance){
        //t.ss();
    }
}
