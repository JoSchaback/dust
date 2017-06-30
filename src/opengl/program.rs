extern crate gl;
extern crate glutin;
//extern crate libc;

use std::ptr;
use std::mem;
use linalg::{Matrix4, Vector4, Matrix3};
use std;

#[allow(dead_code)]
pub struct Program {
    id : gl::types::GLuint,
}

#[allow(dead_code)]
impl Program {
    pub fn new(vertex_shader_code: &[u8], fragment_shader_code: &[u8]) -> Program {
        let vertex_shader   = Program::create_shader(vertex_shader_code,   gl::VERTEX_SHADER);
        let fragment_shader = Program::create_shader(fragment_shader_code, gl::FRAGMENT_SHADER);
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::UseProgram(program);

            Program {
                id: program
            }
        }
    }

    fn create_shader(code: &[u8], shader_type: gl::types::GLenum) -> gl::types::GLuint {
        unsafe {
            let vs = gl::CreateShader(shader_type);
            gl::ShaderSource(vs, 1, [code.as_ptr() as *const _].as_ptr(), ptr::null());
            gl::CompileShader(vs);
            let mut success = mem::uninitialized();
            gl::GetShaderiv(vs, gl::COMPILE_STATUS, &mut success);
            if success == gl::FALSE as gl::types::GLint {
                //let s: [u8, ..code.len()] = code;


                let mut error_log_size = mem::uninitialized();
                gl::GetShaderiv(vs, gl::INFO_LOG_LENGTH, &mut error_log_size);

                if error_log_size > 0 {
                    let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                    gl::GetShaderInfoLog(vs, error_log_size, &mut error_log_size,
                                         error_log.as_mut_ptr() as *mut gl::types::GLchar);
                    error_log.set_len(error_log_size as usize);
                    let code_as_utf8 = std::str::from_utf8(code).unwrap();

                    // maybe use std::slice::from_raw_parts(ptr, len);
                    // like here: https://doc.rust-lang.org/std/primitive.str.html


                    match String::from_utf8(error_log) {
                        Ok(msg) => panic!("error while compiling one of the shaders.\n\
                        ========[ error message ]========\n\
                        {}\
                        ========[ code ]========\n\
                        {}\n\
                        ", msg, code_as_utf8),
                        Err(_) => panic!("Could not convert the log message to UTF-8")
                    }
                }
            }
            //println!("success {:?}", success);
            return vs;
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn attrib_location(&self, name: &str) -> Result<u8, String> {
        // adding delimiter. That is actually dangerous if the delimiter is already appended
        let mut name_with_delimiter = name.to_string();
        name_with_delimiter.push('\0');
        unsafe {
            let location = gl::GetAttribLocation(self.id, name_with_delimiter.as_ptr() as *const _);

            if location < 0 {
                return Err(format!("could not find attrib with name '{}' in currently used Program", name));
            }

            return Ok(location as u8);
        }
    }

    pub fn uniform_location(&self, name: &str) -> Result<u8, String> {
        // adding delimiter. That is actually dangerous if the delimiter is already appended
        let mut name_with_delimiter = name.to_string();
        name_with_delimiter.push('\0');
        unsafe {
            let location = gl::GetUniformLocation(self.id, name_with_delimiter.as_ptr() as *const _);

            if location < 0 {
                return Err(format!("could not find uniform location with name '{}' in currently used Program", name));
            }

            return Ok(location as u8);
        }
    }

    pub fn uniform_matrix4fv_by_name(&self, name: &str, matrix: &Matrix4, transpose: bool) {
        let location = self.uniform_location(name).unwrap();
        self.uniform_matrix4fv(location, matrix, transpose);
    }

    pub fn uniform_matrix4fv(&self, location: u8, matrix: &Matrix4, transpose: bool) {
        unsafe {
            gl::UniformMatrix4fv(
                location  as gl::types::GLint,
                1         as gl::types::GLsizei,
                transpose as gl::types::GLboolean,
                matrix.as_ptr()
            );
        }
    }

    pub fn uniform_matrix3fv_by_name(&self, name: &str, matrix: &Matrix3, transpose: bool) {
        let location = self.uniform_location(name).unwrap();
        self.uniform_matrix3fv(location, matrix, transpose);
    }

    pub fn uniform_matrix3fv(&self, location: u8, matrix: &Matrix3, transpose: bool) {
        unsafe {
            gl::UniformMatrix3fv(
                location  as gl::types::GLint,
                1         as gl::types::GLsizei,
                transpose as gl::types::GLboolean,
                matrix.as_ptr()
            );
        }
    }

    pub fn uniform_vector4_by_name(&self, name: &str, vec: &Vector4) {
        let location = self.uniform_location(name).unwrap();
        self.uniform_4f(location, vec.x, vec.y, vec.z, vec.w);
    }


    pub fn uniform_vector4(&self, location: u8, vec: &Vector4) {
        self.uniform_4f(location, vec.x, vec.y, vec.z, vec.w);
    }

    pub fn uniform_4f(&self, location: u8, x: f32, y: f32, z: f32, w: f32) {
        unsafe {
            gl::Uniform4f(
                location  as gl::types::GLint,
                x as gl::types::GLfloat,
                y as gl::types::GLfloat,
                z as gl::types::GLfloat,
                w as gl::types::GLfloat,
            );
        }
    }

    pub fn uniform_3f(&self, location: u8, x: f32, y: f32, z: f32) {
        unsafe {
            gl::Uniform3f(
                location  as gl::types::GLint,
                x as gl::types::GLfloat,
                y as gl::types::GLfloat,
                z as gl::types::GLfloat,
            );
        }
    }

    pub fn uniform_1i_by_name(&self, name: &str, value: i32) {
        let location = self.uniform_location(name).unwrap();
        self.uniform_1i(location, value);
    }

    pub fn uniform_1i(&self, location: u8, value: i32) {
        unsafe {
            gl::Uniform1i(location as gl::types::GLint, value);
        }
    }

/*
    public void uniform1i(String uniformName, int value) {
ActiveAttrib uniform = uniforms.get(uniformName);
glUniform1i(uniform.location(), value);
}
*/
    // glUniformMatrix4fv(uniform.location(), transpose, floatBufferCache16);

}