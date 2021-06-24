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
