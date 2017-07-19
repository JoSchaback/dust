extern crate gl;
extern crate glutin;
extern crate dust;

mod util;

use glutin::GlContext;

use dust::opengl::{Texture, AttribArrayBuilder, AttribType, Sprite, Font};
use dust::opengl::mesh::{Face,Mesh};
use dust::opengl::program::Program;
use dust::linalg::{Matrix4, Vector3};
use dust::linalg;
use dust::opengl;
use std::boxed::Box;

const VERTEX_SHADER_SRC : &'static [u8] = b"
#version 100
precision highp float;
uniform mat4 projection;
uniform mat4 modelView;

attribute vec3 position;
attribute vec2 uv;

varying vec2 v_uv;

void main() {
    gl_Position = projection * modelView * vec4(position, 1.0);
    v_uv = uv;
}
\0";

const FRAGMENT_SHADER_SRC : &'static [u8] = b"
#version 100
precision highp float;
uniform sampler2D tex;

varying vec2 v_uv;
void main() {
    gl_FragColor = texture2D(tex, v_uv);
}
\0";


fn main() {
    println!("started!");
    let (mut events_loop, gl_window) = util::init("Rendering Text");

    let program = Program::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

    //let map = Sprite::load_fnt_file("assets/font.fnt");

    let tex = Texture::from_pnm_file("assets/font.pnm");

    let font = Font::new(tex, "assets/font.fnt");

    let mesh = font.mesh("Hello,\nthis is a multiline example!\nit works great.");

    let vbo = &mesh.to_element_array_buffer_vbo();

    vbo.bind();

    let position = mesh.attribs().by_name("position").unwrap();
    let uv       = mesh.attribs().by_name("uv").unwrap();

    position.attrib_array_pointer( program.attrib_location("position").unwrap() ) ;
    uv.attrib_array_pointer( program.attrib_location("uv").unwrap() ) ;

    let (width, height) = gl_window.get_inner_size_pixels().unwrap();

    let mut projection = Box::new(Matrix4::new());
    projection.projection(45.0, width as f32, height as f32, 0.1, 100.0);

    program.uniform_matrix4fv_by_name("projection", &projection, false);


    font.tex().bind();

    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
    }

    program.uniform_1i_by_name("tex", 0);

    let mut view = Matrix4::new();
    view.look_at(&Vector3::new(0.0, -2.0, 3.0), linalg::ZERO, linalg::Z_UP);

    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);

        gl::Enable(gl::DEPTH_TEST);

        gl::ClearColor(0.0, 0.5, 0.5, 1.0);
    }

    let mut model     = Matrix4::new();
    model.translation(-1.0, 0.0, 0.0);

    let mut modelview = Matrix4::new();

    let mut running = true;

    while running {

        events_loop.poll_events(|event| {
            running = util::continue_running(event);
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            modelview.copy(&view);
            modelview.mult(&model);

            &program.uniform_matrix4fv_by_name("modelView", &modelview, false);
            &vbo.draw();

            opengl::error();
        }

        let _ = gl_window.swap_buffers().unwrap();
    }
}

