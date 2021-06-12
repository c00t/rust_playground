/// This series of tutorials are somewhat out of date, so check official examples to use gfx-hal correctly.
/// and method used in this tutorial is error-prone.
/// though finally workable.

use std::{borrow::Borrow, iter, mem::ManuallyDrop};

use gfx_hal::{Instance, command::RenderAttachmentInfo, device::{Device, NagaShader, ShaderError}, prelude::Queue, window::PresentationSurface};


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
        //part2
        use gfx_hal::pso::ShaderStageFlags;
        let push_constant_bytes = std::mem::size_of::<PushConstants>() as u32;
        device
            .create_pipeline_layout(iter::empty(), iter::once((ShaderStageFlags::VERTEX,0..push_constant_bytes)))
            .expect("Out of memory.")

        //part1
        // device
        //     .create_pipeline_layout(iter::empty(), iter::empty())
        //     .expect("Out of memory.")
    };

    // get the pipeline
    let pipeline = unsafe {
        //part1
        // make_pipeline::<back::Backend>(&device, &render_pass, &pipeline_layout, compile_shader::<back::Backend>(&device,"shaders/vert1.spv").unwrap(), compile_shader::<back::Backend>(&device,"shaders/frag1.spv").unwrap())
        //part2
        make_pipeline::<back::Backend>(&device, &render_pass, &pipeline_layout, compile_shader::<back::Backend>(&device,"shaders/vert2.spv").unwrap(), compile_shader::<back::Backend>(&device,"shaders/frag2.spv").unwrap())
    };

    // sync primitives
    let submission_complete_fence = device.create_fence(true).expect("Out of memeory");
    let rendering_complete_semaphore = device.create_semaphore().expect("Out of memory");

    //create ResourcesHolder
    let mut resources_holder : ResourcesHolder<back::Backend> = 
        ResourcesHolder(ManuallyDrop::new(Resources{
            instance,
            surface,
            device,
            render_passes:vec![render_pass],
            pipeline_layouts:vec![pipeline_layout],
            pipelines:vec![pipeline],
            command_pool,
            submission_complete_fence,
            rendering_complete_semaphore,
        }));
    
    //part2
    //create a time parameter
    let start_time = std::time::Instant::now();

    //part2

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
                //Resources just holder them, we should use reference to use them
                let res:&mut Resources<_> = &mut resources_holder.0;
                let render_pass = & res.render_passes[0];
                let pipeline = & res.pipelines[0];

                //part2
                //use pipeline layout to tell render pass should use which data in specific shader stage
                let pipeline_layout = & res.pipeline_layouts[0];

                //prepare some vertex specific data to use, animated
                let anim = start_time.elapsed().as_secs_f32().sin()*0.5 + 0.5;

                let small = [0.33,0.33];//scale

                let triangles = &[
                    // Red triangle
                    PushConstants {
                        color: [1.0, 0.0, 0.0, 1.0],
                        pos: [-0.5, -0.5],
                        scale: small,
                    },
                    // Green triangle
                    PushConstants {
                        color: [0.0, 1.0, 0.0, 1.0],
                        pos: [0.0, -0.5],
                        scale: small,
                    },
                    // Blue triangle
                    PushConstants {
                        color: [0.0, 0.0, 1.0, 1.0],
                        pos: [0.5, -0.5],
                        scale: small,
                    },
                    // Blue <-> cyan animated triangle
                    PushConstants {
                        color: [0.0, anim, 1.0, 1.0],
                        pos: [-0.5, 0.5],
                        scale: small,
                    },
                    // Down <-> up animated triangle
                    PushConstants {
                        color: [1.0, 1.0, 1.0, 1.0],
                        pos: [0.0, 0.5 - anim * 0.5],
                        scale: small,
                    },
                    // Small <-> big animated triangle
                    PushConstants {
                        color: [1.0, 1.0, 1.0, 1.0],
                        pos: [0.5, 0.5],
                        scale: [0.33 + anim * 0.33, 0.33 + anim * 0.33],
                    },
                ];
                //part2


                // first use of fence
                // reset our command buffer
                unsafe {
                    use gfx_hal::pool::CommandPool;

                    // add a timeout(1s), we refuse to wait more then a second, avoid hanging.
                    let render_timeout_ns = 1_000_000_000;
                    
                    res.device
                        .wait_for_fence(&res.submission_complete_fence, render_timeout_ns)// block method, when previous command buffer submit, we'll be singeled.
                        .expect("out of Memory or device lost.");
                    
                    res.device
                        .reset_fence(&mut res.submission_complete_fence)// when singaled must reset it.
                        .expect("Out of memory");
                    
                    res.command_pool.reset(false);

                }
                use gfx_hal::window::SwapchainConfig;
                let caps = res.surface.capabilities(&adapter.physical_device);

                let mut swapchain_config = 
                    SwapchainConfig::from_caps(&caps, surface_color_format, surface_extent); // physical size
                
                let surface_image = swapchain_config.framebuffer_attachment();


                if should_configure_swapchain {

                    
                    // fix slowdown issue in MacOs
                    if caps.image_count.contains(&3) {
                        swapchain_config.image_count = 3;
                    }
                    // We also store the surface_extent that was returned in our swapchain_config
                    // - just in case it’s different from the desired size that we provided.
                    surface_extent = swapchain_config.extent;

                    unsafe {
                        res.surface
                            .configure_swapchain(&res.device, swapchain_config)
                            .expect("Failed to configure swapchain.");
                    }

                    should_configure_swapchain = false;

                    // let surface_image = unsafe {
                    //     // refuse timeout
                    //     let acquire_timeout_ns = 1_000_000_000;
    
                    //     match res.surface.acquire_image(acquire_timeout_ns) {
                    //         Ok((image,_)) => image,
                    //         Err(_) => {
                    //             should_configure_swapchain = true;
                    //             return; // we can return nothing here, and ignore the type of other arm.
                    //         }
                    //     }
                    // };
                }
                let surface_image2 = unsafe {
                    match res.surface.acquire_image(!0) {
                        Ok((image, _)) => image,
                        Err(_) => {
                            should_configure_swapchain = true;
                            return;
                        }
                    }
                };
                let framebuffer = unsafe {
                    use std::borrow::Borrow;
                    use gfx_hal::image::Extent;

                    res.device
                        .create_framebuffer(render_pass,iter::once(surface_image), Extent{
                            width:surface_extent.width,
                            height:surface_extent.height,
                            depth:1,
                        }).unwrap()
                };

                //create a viewport settings
                let viewport = {
                    use gfx_hal::pso::{Rect,Viewport};

                    Viewport {
                        rect:Rect {
                            x:0,
                            y:0,
                            w:surface_extent.width as i16,
                            h:surface_extent.height as i16,
                        },
                        depth:0.0..1.0,
                    }
                };
                unsafe {
                    use gfx_hal::command::{
                        ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, SubpassContents,
                    };
                    command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

                    command_buffer.set_viewports(0, iter::once(viewport.clone()));
                    command_buffer.set_scissors(0, iter::once(viewport.rect));

                    command_buffer.begin_render_pass(
                        render_pass,
                        &framebuffer,
                        viewport.rect,
                        iter::once(
                            RenderAttachmentInfo {
                                image_view: surface_image2.borrow(),
                                clear_value: ClearValue {
                                    color: ClearColor{
                                        float32:[0.0,0.0,0.0,1.0],
                                    }
                                }
                            }
                        ),
                        SubpassContents::Inline,
                    );
                    command_buffer.bind_graphics_pipeline(pipeline);

                    //part1
                    // command_buffer.draw(0..3, 0..1); // insatnces used for gpu instance
                    //part1

                    //part2
                    // now render more triangles
                    for triangle in triangles {
                        use gfx_hal::pso::ShaderStageFlags;

                        command_buffer.push_graphics_constants(pipeline_layout, ShaderStageFlags::VERTEX, 0, push_constant_bytes(triangle));

                        command_buffer.draw(0..3,0..1);
                    }
                    //part2

                    command_buffer.end_render_pass();
                    command_buffer.finish();
                }

                unsafe {

                    queue.queues[0].submit(iter::once(&command_buffer), iter::empty(), iter::once(&res.rendering_complete_semaphore), Some(&mut res.submission_complete_fence));

                    //call present ,then finished with a image to screen
                    let result = queue.queues[0].present(&mut res.surface, surface_image2, Some(&mut res.rendering_complete_semaphore));

                    should_configure_swapchain |= result.is_err();

                    // why cache buffer?
                    res.device.destroy_framebuffer(framebuffer);
                }
            },
            _ => (),
        }
    });


}
//part2: define push constant structure
#[repr(C)]
#[derive(Debug,Clone, Copy)]
struct PushConstants{
    color:[f32;4],
    pos:[f32;2],
    scale:[f32;2],
}
// return memory len/map aligned with 32bit
unsafe fn push_constant_bytes<T>(push_constants:&T) -> &[u32]{
    let size_in_byte = std::mem::size_of::<T>();
    let size_in_u32s = size_in_byte/std::mem::size_of::<u32>();

    let start_ptr = push_constants as *const T as *const u32;

    std::slice::from_raw_parts(start_ptr,size_in_u32s)
}
//part2

// Create a Struct to store many resources
struct Resources<B:gfx_hal::Backend> {
    instance:B::Instance,
    surface:B::Surface,
    device:B::Device,
    render_passes:Vec<B::RenderPass>,
    pipeline_layouts:Vec<B::PipelineLayout>,
    pipelines:Vec<B::GraphicsPipeline>,
    command_pool:B::CommandPool,
    submission_complete_fence:B::Fence,
    rendering_complete_semaphore:B::Semaphore,
}
struct ResourcesHolder<B:gfx_hal::Backend>(ManuallyDrop<Resources<B>>);

impl<B:gfx_hal::Backend> Drop for ResourcesHolder<B>{
    fn drop(&mut self) {
        unsafe {
            let Resources {
                instance,
                mut surface,
                device,
                render_passes,
                pipeline_layouts,
                pipelines,
                command_pool,
                submission_complete_fence,
                rendering_complete_semaphore,
            } = ManuallyDrop::take(&mut self.0);
            // destroy things by order.
            device.destroy_semaphore(rendering_complete_semaphore);
            device.destroy_fence(submission_complete_fence);

            for pipeline in pipelines{
                device.destroy_graphics_pipeline(pipeline);
            }
            for pipeline_layout in pipeline_layouts{
                device.destroy_pipeline_layout(pipeline_layout);
            }
            for render_pass in render_passes{
                device.destroy_render_pass(render_pass);
            }
            device.destroy_command_pool(command_pool);
            // surface should first unconfigure the swapchain
            surface.unconfigure_swapchain(&device);
            // surface in fact is as the father of device
            // instance -> surface\adapter
            // adapter -> device\queue family
            instance.destroy_surface(surface);
        }
    }
}

fn compile_shader<B:gfx_hal::Backend>(device:&B::Device,glslpath:&str) -> Result<B::ShaderModule,ShaderError>{
    println!("{}",glslpath);
    let mut file = std::fs::File::open(glslpath).unwrap();

    let spirv = 
        gfx_auxil::read_spirv(&mut file)
        .unwrap();
    unsafe {device.create_shader_module(&spirv)}
}

// building a pipeline example, future may have multiple pipeline, define a function to create a pipeline
unsafe fn make_pipeline<B:gfx_hal::Backend>( // Generic over any backends.
    device:&B::Device,
    render_pass:&B::RenderPass,
    pipeline_layout:&B::PipelineLayout,
    vertex_shader:B::ShaderModule,
    fragment_shader:B::ShaderModule,
) -> B::GraphicsPipeline{
    use gfx_hal::pass::Subpass;
    use gfx_hal::pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, Face, GraphicsPipelineDesc,
        InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, Specialization,
    };
    
    // The EntryPoint struct is exactly what it sounds like - it defines how your shader begins executing.
    let (vs_entry,fs_entry) : (EntryPoint<'_, B>, EntryPoint<'_, B>)= (
        EntryPoint{
            entry:"main",//entry name of the shader function
            module:&vertex_shader,
            specialization:Specialization::default(),
        },
        EntryPoint{
            entry:"main",
            module:&fragment_shader,
            specialization:Specialization::default(),
        },
    );

    // Define a primitive assembler, take vertices -> output primitives(some triangles)
    // Here we define every stages used in graphic pipeline.
    let primitive_assembler = PrimitiveAssemblerDesc::Vertex{
        buffers:&[],
        attributes:&[],
        input_assembler:InputAssemblerDesc::new(Primitive::TriangleList),
        vertex:vs_entry,
        tessellation:None,
        geometry:None,
    };

    // configure the pipeline
    let mut pipeline_desc = GraphicsPipelineDesc::new(
        primitive_assembler,
        Rasterizer{
            cull_face : Face::BACK,
            ..Rasterizer::FILL // Can't add comma here
        },
        Some(fs_entry),
        pipeline_layout,
        Subpass{
            index:0,
            main_pass:render_pass,
        },
    );

    pipeline_desc.blender.targets.push(ColorBlendDesc {
        mask:ColorMask::ALL, // write to all color channel
        blend:Some(BlendState::ALPHA), // alpha blending where pixels overlap
    });

    // create a pipeline
    let pipeline = device
        .create_graphics_pipeline(&&pipeline_desc, None)
        .expect("Failed to create graphic pipeline.");
    
    device.destroy_shader_module(vertex_shader);
    device.destroy_shader_module(fragment_shader);

    pipeline

}
