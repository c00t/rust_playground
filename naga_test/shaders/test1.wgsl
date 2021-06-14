struct MyInputs{
    [[location(0)]] x:vec4<f32>;
    [[builtin(front_facing)]] y:bool;
    [[location(1)]] z:u32;
};
// fn add_one(x : ptr<private,i32>){
//     *x = *x + 1;
// }
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







