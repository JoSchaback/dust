extern crate gl;

pub mod primitives;
pub mod program;
pub mod mesh;

use std::mem;

/// Type of a Vertex Attribute, used by Mesh to know which parts of the Vertex vectors hold the
/// position, normal, UV, etc.
///
/// A Vertex is a Vector of floats where all vertex data () is aligned in an interleaved way.
/// In order to define the structure programatically, Vertex Attributes describe which parts of
/// the Vertex Vector belong to which Attribute, such as Position, Normal, Color or custom data
/// that you may need for crazy stuff in your shaders. The Attribute Type is necessary to interpret
/// the Vertex vector semantically, meaning that for example Mesh can transform, scale, rotate and
/// perform other operations on the data, which it could not if it wouldn't know which parts of
/// the Vertex vector to change.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AttribType {
    Position,
    Normal,
    ColorRgb,
    ColorRgba,
    Uv,
    Custom,
}


/// A Vertex Attribute (long for Attrib) describes a part of an interleaved array, the Vertex.
/// That can be, for example, the three dimensions position, normal or two dimensional UV coords.
pub struct Attrib {
    name    : String,
    length  : u8,
    offset  : usize,
    stride  : usize,
    a_type  : AttribType,
}

impl Attrib {

    pub fn attrib_array_pointer(&self, program_location: u8) {
        unsafe {
            let dimensions = self.stride as usize * mem::size_of::<f32>();
            let location   = program_location as gl::types::GLuint;

            gl::VertexAttribPointer(location, self.length as gl::types::GLint, gl::FLOAT, 0,
                                    dimensions as gl::types::GLsizei,
                                    (self.offset * mem::size_of::<f32>()) as *const () as *const _);
            gl::EnableVertexAttribArray(location);
        }
    }
}

impl Clone for Attrib {

    fn clone(&self) -> Attrib {
        Attrib {
            name   : self.name.clone(),
            length : self.length,
            offset : self.offset,
            stride : self.stride,
            a_type : self.a_type,
        }
    }

    fn clone_from(&mut self, source: &Attrib) {
        self.name   = source.name.clone();
        self.length = source.length;
        self.offset = source.offset;
        self.stride = source.stride;
        self.a_type = source.a_type;
    }
}

#[allow(dead_code)]
pub struct AttribArray {
    attribs: Vec<Attrib>,
    stride: u8,
}

impl Clone for AttribArray {

    fn clone(&self) -> AttribArray {
        AttribArray {
            attribs : self.attribs.clone(),
            stride  : self.stride,
        }
    }

    fn clone_from(&mut self, source: &AttribArray) {
        self.attribs = source.attribs.clone();
        self.stride  = source.stride;
    }
}

#[allow(dead_code)]
impl AttribArray {

    pub fn stride(&self) -> usize {
        self.stride as usize
    }

    pub fn by_index(&self, index: usize) -> &Attrib {
        &self.attribs[index]
    }

    pub fn by_name(&self, name: &str) -> Result<&Attrib, String> {

        for attrib in &self.attribs {
            if attrib.name.eq(&name) {
                return Ok(attrib);
            }
        }

        Err(format!("could not find Attrib with name '{}' in AttribArray", name))
    }

    pub fn by_type(&self, a_type: AttribType) -> Result<&Attrib, String> {

        for attrib in &self.attribs {
            if attrib.a_type == a_type {
                return Ok(attrib);
            }
        }

        Err(format!("could not find Attrib with type '{:?}' in AttribArray", a_type))
    }

}


pub struct AttribArrayBuilder {
    names  : Vec<String>,
    types  : Vec<AttribType>,
    lengths: Vec<usize>,
    stride : usize,
}

impl AttribArrayBuilder {

    pub fn new() -> AttribArrayBuilder {
        AttribArrayBuilder {
            names  : Vec::new(),
            lengths: Vec::new(),
            types  : Vec::new(),
            stride : 0,
        }
    }

    pub fn push_custom(self, name: &str, len: u8) -> AttribArrayBuilder {
        self.push(name, len, AttribType::Custom)
    }

    pub fn push(mut self, name: &str, len: u8, a_type: AttribType) -> AttribArrayBuilder {
        self.names.push(name.to_string());
        self.lengths.push(len as usize);
        self.types.push(a_type);
        self.stride += len as usize;
        self
    }

    pub fn build(self) -> AttribArray {

        let mut attribs : Vec<Attrib> = Vec::with_capacity(self.names.len());
        let mut offset : usize = 0;

        for i in 0..self.names.len() {
            attribs.push( Attrib {
                name     : self.names[i].clone(),
                length   : self.lengths[i] as u8,
                a_type   : self.types[i],
                offset   : offset,
                stride   : self.stride,
            });
            offset += self.lengths[i];
        }

        AttribArray {
            attribs: attribs,
            stride : self.stride as u8,
        }
    }

}

/// Wrapper struct for VertexBufferObjects. A VertexBufferObject (VBO) is a memory region on the
/// graphics card used for rendering geometry. It wraps an ARRAY_BUFFER for Vertex data plus an optional
/// ELEMENT_ARRAY_BUFFER to store Vertex indices.
#[allow(dead_code)]
pub struct VertexBufferObject {
    array_buffer_id           : gl::types::GLuint,
    element_array_buffer_id   : Option<gl::types::GLuint>,
    element_array_buffer_size : i32,
    vertex_count              : i32,
}

impl VertexBufferObject {

    pub fn new(array_buffer_id           : gl::types::GLuint,
               element_array_buffer_id   : Option<gl::types::GLuint>,
               element_array_buffer_size : i32,
               vertex_count              : i32) -> VertexBufferObject {
        VertexBufferObject {
            array_buffer_id           : array_buffer_id,
            element_array_buffer_id   : element_array_buffer_id,
            element_array_buffer_size : element_array_buffer_size,
            vertex_count              : vertex_count,
        }
    }

    pub fn from_vec_as_array_buffer(vec: &Vec<f32>, stride: usize) -> VertexBufferObject {
        unsafe {
            let mut vb = mem::uninitialized();
            gl::GenBuffers(1, &mut vb);
            gl::BindBuffer(gl::ARRAY_BUFFER, vb);
            let buffer_size = vec.len() * mem::size_of::<f32>();

            gl::BufferData(gl::ARRAY_BUFFER, buffer_size as gl::types::GLsizeiptr, vec.as_ptr() as *const _, gl::STATIC_DRAW);

            VertexBufferObject::new(vb, None, 0, (vec.len() / stride) as i32)
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.array_buffer_id);

            if self.element_array_buffer_id.is_some() {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.element_array_buffer_id.unwrap());
            }
        }
    }

    pub fn draw(&self) {
        unsafe {
            if self.element_array_buffer_id.is_some() {

                use std::ptr;

                gl::DrawElements(gl::TRIANGLES, self.element_array_buffer_size, gl::UNSIGNED_SHORT, ptr::null());
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
            }
        }
    }
}

/// checks for OpenGL errors
pub fn error() {
    unsafe {
        let error = gl::GetError();
        if error != 0 {
            panic!("OpenGL error {}", error);
        }

    }
}

/// Texture 2d wrapper
#[allow(dead_code)]
pub struct Texture {
    id     : gl::types::GLuint,
    width  : u32,
    height : u32,
}

#[allow(dead_code)]
impl Texture {
    pub fn new(width:u32, height:u32, rgb_raster: Vec<u8>) -> Texture {
        unsafe {
            let mut vb = mem::uninitialized();
            gl::GenTextures(1, &mut vb);
            gl::BindTexture(gl::TEXTURE_2D, vb);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, rgb_raster.as_ptr() as *const _);
            //glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0, GL_RGBA, GL_UNSIGNED_BYTE, buf);

            Texture::parameters(gl::LINEAR, gl::LINEAR, gl::CLAMP_TO_EDGE, gl::CLAMP_TO_EDGE);

            // unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);

            Texture {
                id    : vb,
                width : width,
                height: height,
            }
        }
    }

    pub fn parameters(mag_filter: gl::types::GLenum,
                      min_filter: gl::types::GLenum,
                      wrap_s    : gl::types::GLenum,
                      wrap_t    : gl::types::GLenum,) {

        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    fn parse_ascii_line(buffer : &Vec<u8>, index: usize) -> (String, usize) {
        let mut i = index;
        let mut string = String::new();

        while buffer[i] != 10 {
            string.push( buffer[i] as char );
            i += 1;
        }

        (string, i)
    }

    pub fn from_pnm_file(filename: &str) -> Texture {

        use std::error::Error;
        use std::fs::File;
        use std::io::BufReader;
        use std::io::prelude::*;
        use std::path::Path;

        // Create a path to the desired file
        let path = Path::new(filename);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display,
                               why.description()),
            Ok(file) => file,
        };

        let mut buf_reader = BufReader::new(file);
        let mut buffer     = Vec::new();
        let result         = buf_reader.read_to_end(&mut buffer);

        if result.is_err() {
            panic!("could not read file {}, because of {}", filename, result.err().unwrap());
        }

        let (magic_number_as_string, i) = Texture::parse_ascii_line(&buffer, 0);
        let mut index = i + 1; // jump over carriage return

        assert_eq!(magic_number_as_string, "P6", "magic number in file {} is expected to be P6, but appears to be {}", filename, magic_number_as_string);

        // skip commentary line, ASCII
        let (_, i) = Texture::parse_ascii_line(&buffer, index);
        index = i + 1;

        // read size as ASCII
        let (size_string, i) = Texture::parse_ascii_line(&buffer, index);
        index = i + 1;

        // the size string comes in the format "256 256" and this needs to be split in to
        // in order to parse it into width and height
        let split_vec : Vec<&str> = size_string.split(" ").collect();

        // width and height of the image to be loaded in pixels
        let width  = split_vec[0].parse::<u32>().unwrap();
        let height = split_vec[1].parse::<u32>().unwrap();

        let (max_value_string, i) = Texture::parse_ascii_line(&buffer, index);
        index = i + 1;

        assert_eq!(max_value_string, "255", "while parsing the PNM file, I expect the 'max value' to be 255, but it turned out to be {}", max_value_string);

        let size_in_bytes = (width * height * 3) as usize;

        let mut rgb_image : Vec<u8> = Vec::with_capacity( size_in_bytes );

        while index < buffer.len() {
            rgb_image.push( buffer[index] );
            index += 1;
        }

        Texture::new(width, height, rgb_image)
    }
}
