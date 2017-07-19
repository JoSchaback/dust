extern crate glutin;
extern crate gl;

use glutin::{GlContext, EventsLoop};
use dust::opengl;

/// initiatlize glutin window
pub fn init(title: &str) -> (EventsLoop, glutin::GlWindow){

    let events_loop = EventsLoop::new();

    let window    = glutin::WindowBuilder::new().with_title(title);
    let context   = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    opengl::error();

    (events_loop, gl_window)
}

/// handle default window and keystroke events for closing example program
pub fn continue_running(event1: glutin::Event) -> bool {

    use glutin::{ControlFlow, WindowEvent, VirtualKeyCode};

    let mut continue_running = true;

    match event1 {
        glutin::Event::WindowEvent { event, .. } => match event {
            glutin::WindowEvent::Closed => {
                continue_running = false;
            },
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    continue_running = false;
                }
            },
            /*
                        glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => {
                            //println!("GRRR");
                            return true;
                        },*/

            _ => ()
        },

        _ => ()
    }

    continue_running
}