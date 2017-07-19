extern crate glutin;
extern crate gl;

use glutin::GlContext;
use dust::opengl;

/// initiatlize glutin window
pub fn init(title: &str) -> (glutin::EventsLoop, glutin::GlWindow){

    let events_loop = glutin::EventsLoop::new();

    let window    = glutin::WindowBuilder::new().with_title(title);
    let context   = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    opengl::error();

    (events_loop, gl_window)
}

/// handle default window and keystroke events for closing example program
pub fn shall_stop(event1: glutin::Event) -> glutin::ControlFlow {

    use glutin::{ControlFlow, WindowEvent, VirtualKeyCode};

    match event1 {
        glutin::Event::WindowEvent { event, .. } => match event {
            glutin::WindowEvent::Closed => {
                return ControlFlow::Break;
            },
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    return ControlFlow::Break;
                }
            },
/*
            glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => {
                //println!("GRRR");
                return true;
            },*/

            _ => return ControlFlow::Continue
        },

        _ => return ControlFlow::Continue,
    }

    ControlFlow::Continue
}