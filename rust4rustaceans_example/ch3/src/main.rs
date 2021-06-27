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
