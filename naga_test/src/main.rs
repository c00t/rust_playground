use std::fs;

fn main() {
    let input = fs::read_to_string("shaders/quad.wgsl").unwrap();
    let result = naga::front::wgsl::parse_str(&input);
    let module = match result {
        Ok(v) => v,
        Err(ref e) => {
            e.emit_to_stderr();
            panic!("unable to parse WGSL");
        }
    };
    //validate
    let info = match naga::valid::Validator::new(
        naga::valid::ValidationFlags::all()
    )
    .validate(&module)
    {
        Ok(info) => Some(info),
        Err(error) => {
            panic!("Validation Error.");
        }
    };
    println!("{:#?}",info);

    use naga::back::spv;
    let spv_option:naga::back::spv::Options = Default::default();
    let spv = 
        spv::write_vec(&module, info.as_ref().unwrap(), &spv_option).unwrap();
    
    let bytes = spv
        .iter()
        .fold(Vec::with_capacity(spv.len() * 4), |mut v, w| {
            v.extend_from_slice(&w.to_le_bytes());
            v
        });
    fs::write("shaders/quad.spv",bytes.as_slice()).unwrap();

    //later, rewrite gfx-hal test shaders to verify the spv correctness. 
    
}
