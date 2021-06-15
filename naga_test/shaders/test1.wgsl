struct MyInputs{
    [[location(0)]] x:vec4<f32>;
    [[builtin(front_facing)]] y:bool;
    [[location(1)]] z:u32;
};
// fn add_one(x : ptr<private,i32>){
//     *x = *x + 1;
// }

struct Student{
    grade:i32;
    GPA:f32;
    attendance:array<bool,4>;
};

fn function_scope(){
    //var<function> count:i32;
    var scale:f32 = 1.0;
    var pi:f32 = 3.14159;
    let unit:i32 = 1;
    
    // zero-value?
    //var k:f32 = f32();//currently not support

    // zero value structure
    // var s:Student = Student(); // currently not support
    var s:Student = Student(2,1.7,array<bool,4>(true,true,false,false));

    // access by rgba
    var s:vec2<f32> = vec2<f32>(1.0,2.0);
    var d:vec2<f32> = s.rr;

}
type Arr = array<f32,3>;


[[stage(fragment)]]
fn main(in1:MyInputs) -> [[location(0)]] vec4<f32> {
    let a:f32 = dot(vec2<f32>(0.4,0.4),vec2<f32>(0.2,0.2));
    //2 row 3 colomn or 3 row 2 column
    let b:mat2x3<f32> = mat2x3<f32>(vec3<f32>(1.0,2.0),vec3<f32>(1.0,2.0));

    let i:i32 = 0;
    let j:Arr = Arr(1.0,2.0,3.0);

    //add_one(&i)// value 1
    return vec4<f32>(0.4, 0.4, 0.8, 1.0);
}







