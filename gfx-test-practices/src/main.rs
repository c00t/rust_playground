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
