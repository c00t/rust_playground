[[stage(fragment)]]
fn main() -> [[location(0)]] vec4<f32> {
    let a:f32 = dot(vec2<f32>(0.4,0.4),vec2<f32>(0.2,0.2));
    return vec4<f32>(0.4, 0.4, 0.8, 1.0);
}