mod common;

use winit::{
    event_loop::{EventLoop},
};

fn main() {

    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    
    window.set_title("Using the vertex buffer");
    env_logger::init();
    pollster::block_on( common::run(event_loop, window));
}

