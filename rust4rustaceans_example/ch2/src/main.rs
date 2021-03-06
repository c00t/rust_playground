use std::thread;

fn main() {
    //High Level Model
    //println!("Hello, world!");
    let mut x;
    //assert_eq!(x,42); //error
    x = 42;
    let y = &x;
    //*y = 23; //error
    x = 43;

    //assert_eq!(*y,42); //error, &mut ref vs. & ref 

    //Low Level Model
    

    // spawn 'static bounds
    let x = || {
        println!("zzzzz");
    };

    thread::spawn(x);

    // ownership

    let x1 = 42;
    let y1 = Box::new(84);
    {
        // new scope
        let z = (x1,y1);// x1 is Copied into z, y1 is Moved into z.
    }// z out of scope, drop value from x1,y1
    let x2 = x1;// x1 is Copied into z above, not Moved
    // let y2 = y1;// y1 is Moved to z above.

    // self-references
    let ss = String::from("12345678");
    // let self_r = SelfR{
    //     str_ins:ss,
    //     str_ref:&self_r,
    // };

    let mut s = Box::new(42);
    replace_with_84(&mut s);

    ref_mut_invariant();

    variance_in_generic_lifetime();
}
//variance in generic lifetime
struct MutStr<'a,'b>{
    s: &'a mut &'b str
    //这里使用了两个generic lifetime标记
    //，乍一看我们并没有两个lifetime更改的需求。
    //我们甚至并没有定义返回ref的函数。
    //但是要是只用一个lifetime，那么就会报错
    //1.首先由于struct定义中的任何引用都必须加上'lifetime标记
}
struct RefStr<'a>{
    s:&'a &'a str,
}
fn variance_in_generic_lifetime(){
    let mut s = "hello";
    * MutStr{
        s:&mut s
    }.s = "world";
    //在上面这一行，编译器就要决定其定义中的generic lifetime应该是啥
    //使用两个lifetime时，
    //'a被决定为&mut s这个引用的lifetime。
    //'b被决定为’static，因为其内容"hello"是'static str.
    //使用一个lifetime时，由于'static和‘a的关系，
    //'a被决定为'static，即整个程序中一直存在的exclusive引用。
    println!("{}",s);
    //那么，再调用上面的引用时，使用了shared ref，
    //而且，rust编译器默认的实现中，将&mut T实现为invariance
    //如果没有&mut修饰的话，Rust能够将'static的lifetime缩减至'a
    //最终的表现就是Rust无法将&mut s的lifetime缩减至&'a mut
    //最终决定的lifetime为&'static mut
    //就会出现报错，如果上面这一行没有的话，是不会出现报错的。
}

//&mut invariant usage
struct TestInvariant{
    x:i32,
    y:i32,
}
static S1 : TestInvariant = TestInvariant{x:0,y:0};
static S2 : TestInvariant =  TestInvariant{x:1,y:1};
static S3 : TestInvariant =  TestInvariant{x:2,y:2};
fn ref_mut_invariant(){
    let s = TestInvariant{x:2,y:2};
    let mut test_v = vec![&s];   
    vec_lifetime_a_params(&mut test_v);
}
fn vec_lifetime_a_params<'a>(v:&mut Vec<&'a TestInvariant>){
    v.push(&S3);
}



//self-references
struct SelfR<'a>{
    str_ins:String,
    str_ref:&'a SelfR<'a>,
}

//mutable references optimization
fn noalias(input:&i32,output:&mut i32){
    if *input == 1{
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
    //Rust Compiler think that => single if-else is sufficient.
    //Because that input & output not point to the same location.
}

//move a value dehind the mutable reference
fn replace_with_84(s:&mut Box<i32>){
    // ilegal: after assign, *s will move to was, leaving *s empty
    // let was = *s; 当然，这里其实rust check也会报错的
    let was = std::mem::take(s);

    *s = was;// default value dropped immediately.
    // exchange values behind &mut
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r); // *r is 42, *s is 84

}

// sample code for lifetime 
fn simple_sample_for_lifetime(){
    let mut x = Box::new(42);
    let r = &x; //'a
    if 1.0 > 0.5{
        *x = 84;
        //auto deref to &mut, and check that no conflict here.
        //because that r doesn't be used in this branch.
        //compiler is smart enough.
    }else{
        println!("{}",r); //'a
    }
    // println!("{}",r);//'a 
    // doesn't compile,the `lifetime` line will flow through two branchs, and conflict at `*x = 84`
    // which use &mut
    // 其实这也就解释了，为啥报错会报在*x=84这一行，按照每遇到一次引用就返回引用taken点的检查方法，
    // 确实应该在*x = 84 这一行报出故障。
}

fn simple_sample_for_lifetime_more(){
    let mut x = Box::new(42);
    let mut z = &x; //'a1
    for i in 0..100 {
        println!("{}",z); //'a1,...,'a98 ,when checked, no confilict
        x = Box::new(i); //'a1 end here, 'a2 end here, 'a98 end here,&mut
        z = &x; //lifetime restart: 'a2 start here, and ends at above line in next loop. 'a99 start here
    }
    println!("{}",z); //'a99 end here
}

// multiple lifetimes
// multiple references, returned value only tied to one of them.
struct StrSplit<'s,'p>{
    delimiter:&'p str,
    document:&'s str,
}
impl<'s,'p> Iterator for StrSplit<'s,'p>{
    type Item = &'s str;
    // return type only rely on `document`
    // we don't care about `delimiter`

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
fn str_defore(s:&str,c:char)->Option<&str>{
    StrSplit{
        document:s,
        delimiter:&c.to_string(),
    }.next()
}



// error example
struct StrSplit2<'s>{
    delimiter:&'s str,
    document:&'s str,
}
impl<'s> Iterator for StrSplit2<'s>{
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
// fn str_defore2(s:&str,c:char)->Option<&str>{
//     // StrSplit2{
//     //     document:s,
//     //     delimiter:&c.to_string(),
//     // }.next()
//     // flow from ( c -> <return value> ), borrow checker check that and reject. 
//     // lifetime of <s> and <return vlue> is the same.
// }