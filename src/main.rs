#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod core;
mod window;
mod utils;

use winit::event_loop::EventLoop;
use crate::window::app::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
