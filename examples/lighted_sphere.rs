extern crate gl;
extern crate glutin;
extern crate dust;

use dust::opengl::program::Program;
use dust::linalg::{Matrix4, Vector3, Vector4, Matrix3};
use dust::linalg;
use dust::opengl;

/// Draws a colored sphere, lit by a single light source
fn main() {
    println!("started!");
    let events_loop = glutin::EventsLoop::new();

    let window = glutin::WindowBuilder::new()
        .with_title("Lighted Sphere".to_string())
        .with_dimensions(1024, 768)
        .with_vsync()
        .build(&events_loop)
        .unwrap();

    unsafe {
        window.make_current()
    }.unwrap();

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    opengl::error();

    let shader = LightShader::new();

    let mesh = opengl::primitives::icosphere(3 as u8);

    let vbo = &mesh.to_element_array_buffer_vbo();

    vbo.bind();

    let position = &mesh.attribs().by_name("position").unwrap();
    let color    = &mesh.attribs().by_name("color").unwrap();
    let normal   = &mesh.attribs().by_name("normal").unwrap();

    position.attrib_array_pointer( shader.position_attrib_loc ) ;
    color.attrib_array_pointer( shader.color_attrib_loc ) ;
    normal.attrib_array_pointer( shader.normal_attrib_loc ) ;

    let (width, height) = window.get_inner_size_pixels().unwrap();

    let mut projection = Matrix4::new();
    projection.projection(45.0, width as f32, height as f32, 0.1, 100.0);
    shader.update_projection(&projection);

    let mut view = Matrix4::new();
    view.look_at(
        &Vector3::new(2.0, 2.0, 2.0),
        linalg::ZERO,
        linalg::Z_UP);

    let mut light_in_view_space = Vector4::new(1.0, 0.0, 0.0, 0.0);
    view.mult_to_vec4(&mut light_in_view_space);
    light_in_view_space.normalize();

    shader.update_light_dir(&light_in_view_space);

    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);

        gl::Enable(gl::DEPTH_TEST);

        gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    }

    // create a model matrix
    let mut model         = Matrix4::new();

    // create a model view matrix
    let mut modelview     = Matrix4::new();

    // create normal matrix
    let mut normal_matrix = Matrix3::new();

    let mut alpha     = 0.0;

    let mut running   = true;
    while running {
        events_loop.poll_events(|event1| {
            //println!("stuff: {:?}", event1);
            match event1 {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => {
                        running = false;
                    },

                    glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) => {
                        //println!("GRRR");
                        running = false;
                    },

                    _ => ()
                }
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            model.rotation(alpha, linalg::Z_UP);

            modelview.copy(&view);
            modelview.mult(&model);

            shader.update_modelview(&modelview);

            normal_matrix.calc_normal_matrix(&modelview);
            shader.update_normal_matrix(&normal_matrix);

            &vbo.draw();

            opengl::error();
        }

        alpha += 0.01;

        window.swap_buffers().unwrap();
    }
}

/// Wrapper struct for shader program. Also serves as cache for uniform and attrib buffer
/// locations.
struct LightShader {
    program: Program,
    projection_uniform_loc: u8,
    modelview_uniform_loc: u8,
    normal_matrix_uniform_loc: u8,
    light_dir_uniform_loc: u8,

    position_attrib_loc: u8,
    normal_attrib_loc: u8,
    color_attrib_loc: u8,
}

impl LightShader {
    fn new() -> LightShader {
        let program = Program::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC);

        program.use_program();

        LightShader {
            projection_uniform_loc    : program.uniform_location("projection").unwrap(),
            modelview_uniform_loc     : program.uniform_location("modelView").unwrap(),
            normal_matrix_uniform_loc : program.uniform_location("normalMatrix").unwrap(),
            light_dir_uniform_loc     : program.uniform_location("lightDirectionInEyeSpace").unwrap(),

            position_attrib_loc : program.attrib_location("position").unwrap(),
            normal_attrib_loc   : program.attrib_location("normal").unwrap(),
            color_attrib_loc    : program.attrib_location("color").unwrap(),
            program: program,
        }
    }

    fn update_projection(&self, projection: &Matrix4) {
        self.program.uniform_matrix4fv(self.projection_uniform_loc, &projection, false);
    }

    fn update_normal_matrix(&self, normal_matrix: &Matrix3) {
        self.program.uniform_matrix3fv(self.normal_matrix_uniform_loc, &normal_matrix, false);
    }

    fn update_modelview(&self, modelview: &Matrix4) {
        self.program.uniform_matrix4fv(self.modelview_uniform_loc, &modelview, false);
    }

    fn update_light_dir(&self, light_dir: &Vector4) {
        self.program.uniform_3f(self.light_dir_uniform_loc, light_dir.x, light_dir.y, light_dir.z);
    }

}


const VERTEX_SHADER_SRC : &'static [u8] = b"
#version 100

uniform mat4 projection;
uniform mat4 modelView;
uniform mat3 normalMatrix;
uniform vec3 lightDirectionInEyeSpace;

attribute vec3 position;
attribute vec3 color;
attribute vec3 normal;

varying vec3 v_color;
varying vec3 v_normal;
varying vec3 v_position;

void main() {
    gl_Position = projection * modelView * vec4(position, 1.0);
    v_normal    = normalize( normalMatrix * normal );           // normal in view space
    v_position  = -vec3( modelView * vec4(position, 1.0) );     // position in view space
    v_color     = color;
}
\0";

const FRAGMENT_SHADER_SRC : &'static [u8] = b"
#version 100

uniform vec3 lightDirectionInEyeSpace;

varying vec3 v_color;
varying vec3 v_normal;
varying vec3 v_position;

void main() {
   // set the specular term to black
   vec4 spec  = vec4(0.0, 0.0, 0.0, 0.0);

   vec3 l_dir = normalize( lightDirectionInEyeSpace );
   vec3 n     = normalize( v_normal );
   vec3 e     = normalize( v_position );

   float intensity = max(dot(n,l_dir), 0.0);

   if (intensity > 0.0) {                          // if the vertex is lit compute the specular color
      vec3 h          = normalize(l_dir + e);      // compute the half vector

      float shininess = 300.0;
      float intSpec   = max(dot(h,n), 0.0);
      vec4 specular   = vec4(1.0, 1.0, 1.0, 1.0);
      spec            = specular * pow(intSpec, shininess);
   }

   vec4 diffuse = vec4(v_color, 1.0);
   vec4 ambient = vec4(v_color*0.1, 1.0);
   gl_FragColor = max(intensity * diffuse + spec, ambient);
}
\0";