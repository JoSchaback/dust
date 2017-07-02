extern crate glutin;
extern crate gl;

use dust::opengl;

/// initiatlize glutin window
pub fn init(title: &str) -> (glutin::EventsLoop, glutin::Window){
    let events_loop = glutin::EventsLoop::new();

    let window = glutin::WindowBuilder::new()
        .with_title(title.to_string())
        .with_dimensions(1024, 768)
        .with_vsync()
        .build(&events_loop)
        .unwrap();

    unsafe {
        window.make_current()
    }.unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    opengl::error();

    (events_loop, window)
}

/// handle default window and keystroke events for closing example program
pub fn shall_stop(event1: glutin::Event) -> bool {
    match event1 {
        glutin::Event::WindowEvent { event, .. } => match event {
            glutin::WindowEvent::Closed => {
                return true;
            },

            glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => {
                //println!("GRRR");
                return true;
            },

            _ => ()
        }
    }

    false
}