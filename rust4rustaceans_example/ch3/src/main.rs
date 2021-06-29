use std::fmt::Display;

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
fn dsts_faults(){
    let x:(i32,dyn Iterator,[u8],i32);// err. Sized not implemented.
}

/// Use `Self:Sized` to limit trait objects
trait qq where Self:Sized{
    fn ss(){
    }
}
fn ss_self_sized(s:&dyn qq){
    //error: qq is not `object-safe`.
}

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
impl ZZ for Point{
    type z = QQ;
    
    fn zprint(&self,other:Self::z) {
        todo!()
    }
}
// will error here
// impl ZZ for Point {
//     type z = BB;

//     fn zprint(&self,other:Self::z) {
//         todo!()
//     }
// }