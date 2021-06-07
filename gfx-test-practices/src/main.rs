use std::iter;

use gfx_hal::device::NagaShader;


#[cfg(feature = "dx12")]
extern crate gfx_backend_dx12 as back;

fn main() {


    use std::mem::ManuallyDrop;

    use gfx_hal::{
        device::Device,
        window::{Extent2D,PresentationSurface,Surface},
        Instance,
    };

    const APP_NAME:&'static str = "Part 1: Drawing a triangle";
    const WINDOW_SIZE:[u32;2] = [512,512];

    let event_loop = winit::event_loop::EventLoop::new();

    //get the logical&physical wiindows size 
    let (logical_window_size,physical_window_size) = {
        use winit::dpi::{LogicalSize,PhysicalSize};

        let dpi = event_loop.primary_monitor().unwrap().scale_factor();
        let logical:LogicalSize<u32> = WINDOW_SIZE.into();
        let physical:PhysicalSize<u32> = logical.to_physical(dpi);

        (logical,physical)
    };
    println!("{:?},{:?}",logical_window_size,physical_window_size);//My dpi scale is 1
    //Create a Extent2D to use later
    let mut surface_extent = Extent2D{
        width:physical_window_size.width,
        height:physical_window_size.height,
    };

    let window = winit::window::WindowBuilder::new()
        .with_title(APP_NAME)
        .with_inner_size(logical_window_size)
        .build(&event_loop)
        .expect("Failed to create window");
    
    // rebuild the swap-chain on the first frame
    // because a frame to display& a frame to render?
    // configure these image-dimensions in swap-chain
    let mut should_configure_swapchain = true;

    //Request Graphics resources
    let (instance,surface,adapter) = {
        let instance = back::Instance::create(APP_NAME, 1).expect("Backend not supported.");

        let surface = unsafe {
            instance.create_surface(&window).expect("Failed to create a surface for render.")// don't use ';' here, will cause return () to surface
        };// instead, use ';' here

        let adapter = instance.enumerate_adapters().remove(0);// get all graphic adapter and then get adapter at index 0

        (instance,surface,adapter)
    };

    // Get logical device, and a queue group. Firstly, should choose a Queue Family, which has different capibility
    let (device, mut queue) = {
        use gfx_hal::queue::QueueFamily;

        let queue_family = adapter.queue_families.iter().find(|family| {
            surface.supports_queue_family(family) && family.queue_type().supports_graphics()
        }).expect("No Compatible Queue Found!");

        let mut gpu = unsafe {
            use gfx_hal::adapter::PhysicalDevice;

            adapter
                .physical_device
                .open(&[(queue_family,&[1.0])], gfx_hal::Features::empty())
                .expect("Failed to get a logical device.")
        };

        (gpu.device,gpu.queue_groups.pop().unwrap())
    };

    // Create a Command_Pool then Create a Primary Command_Buffer to reuse later
    let (command_pool, mut command_buffer) = unsafe {
        use gfx_hal::command::Level;
        use gfx_hal::pool::{CommandPool,CommandPoolCreateFlags};

        let mut command_pool = device
            .create_command_pool(queue.family, CommandPoolCreateFlags::empty())
            .expect("Out of graphic memory.");
        
        let command_buffer = command_pool.allocate_one(Level::Primary);

        (command_pool,command_buffer)
    };
    
    // choose a color format to use with a Render Pass
    let surface_color_format = {
        use gfx_hal::format::{ChannelType,Format};

        let supported_formats = surface
            .supported_formats(&adapter.physical_device)
            .unwrap_or(vec![]);

        let default_format = * supported_formats.get(0).unwrap_or(&Format::Rgb8Srgb);
        // Choose a Srgb format support srgb, then the gpu handle gamma-correction for us.

        supported_formats
            .into_iter()
            .find(|format| format.base_format().1 == ChannelType::Srgb)
            .unwrap_or(default_format)
    };
    // Create a RenderPass, With a color-attachment/subpass
    // a attachment is a slot to be filled with.
    // a subpass defines a subset of those attachments to use.
    let render_pass = {
        use gfx_hal::image::Layout;
        use gfx_hal::pass::{
            Attachment,AttachmentLoadOp,AttachmentOps,AttachmentStoreOp,SubpassDesc
        };

        let color_attachment = Attachment{
            format:Some(surface_color_format),
            samples:1,
            ops:AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),//when color load, clear the attachment. when color store, also store.
            stencil_ops:AttachmentOps::DONT_CARE,// Shadow some area
            layouts:Layout::Undefined..Layout::Present,
        };

        let subpass = SubpassDesc{
            colors: &[(0,Layout::ColorAttachmentOptimal)],
            depth_stencil: None,
            inputs: &[],
            resolves: &[],
            preserves: &[],
        };

        unsafe {
            device
                .create_render_pass(iter::once(color_attachment), iter::once(subpass), iter::empty()) // use std::iter to create temp iterator
                .expect("Out of memory.")
        }
    };

    // Then define our rendering pipeline, 
    let pipeline_layout = unsafe {
        device
            .create_pipeline_layout(iter::empty(), iter::empty())
            .expect("Out of memory.")
    };

    // import shaders
    // shaderc need to build from source, so we need to install sdk, then compile glsl to SPIR-V
    let vertex_shader = include_str!("../shaders/part1.vert");
    let fragment_shader = include_str!("../shaders/part1.frag");
    
    

    // Note that this takes a `move` closure. This means it will take ownership
    // over any resources referenced within. It also means they will be dropped
    // only when the application is quit.
    event_loop.run(move |event,_,control_flow| {
        use winit::event::{Event,WindowEvent};
        use winit::event_loop::ControlFlow;

        match event {
            Event::WindowEvent{event,..} => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(dims) => {
                    // Why after resized, windows is not fullfilled with white.
                    surface_extent = Extent2D{
                        width:dims.width,
                        height:dims.height,
                        
                    };
                    should_configure_swapchain = true;
                },
                WindowEvent::ScaleFactorChanged {new_inner_size,..} => {
                    surface_extent = Extent2D{
                        width:new_inner_size.width,
                        height:new_inner_size.height,
                    };
                    should_configure_swapchain = true;
                },
                _ => (),
            },
            Event::MainEventsCleared => window.request_redraw(),
            Event::RedrawRequested(_) => {
                //we will render our image here
            },
            _ => (),
        }
    });


}
// fn compile_shader(glsl:&str,shader_kind:){

// }