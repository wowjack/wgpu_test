use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window
};


fn main() {
    //create the wgpu instance and print out the available adapters and their backends
    let instances = wgpu::Instance::new(wgpu::Backends::all());
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }

    //create window that handles close
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("Cool Window");
    env_logger::init();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, ..} => *control_flow = ControlFlow::Exit,
            _ => ()
        }
    })
}