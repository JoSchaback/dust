extern crate gl;

pub mod primitives;
pub mod program;
pub mod mesh;

use std::mem;
use std::collections::HashMap;

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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width_as_f32(&self) -> f32 {
        self.width as f32
    }

    pub fn height_as_f32(&self) -> f32 {
        self.height as f32
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
        let width  = split_vec[0].parse::<usize>().unwrap();
        let height = split_vec[1].parse::<usize>().unwrap();

        let (max_value_string, i) = Texture::parse_ascii_line(&buffer, index);
        index = i + 1;

        assert_eq!(max_value_string, "255", "while parsing the PNM file, I expect the 'max value' to be 255, but it turned out to be {}", max_value_string);

        let size_in_bytes = width * height * 3;

        let mut rgb_image : Vec<u8> = Vec::with_capacity( size_in_bytes );

        unsafe { rgb_image.set_len(size_in_bytes); }

        // tracks x and y of source image while reading
        let mut source_x : usize = 0;
        let mut source_y : usize = 0;

        // the index in the target buffer where to write, includes the vertical flip
        //let mut write_index : usize = 0;

        while index < buffer.len() {

            //println!("writing buffer, width: {}, height: {}, source_x: {}, source_y: {}, index: {}, size_in_bytes: {}", width, height, source_x, source_y, index, size_in_bytes);

            // we have to flip the y axis of the image
            rgb_image[ width*(height-source_y-1)*3 + source_x*3 + 0] = buffer[index + 0];
            rgb_image[ width*(height-source_y-1)*3 + source_x*3 + 1] = buffer[index + 1];
            rgb_image[ width*(height-source_y-1)*3 + source_x*3 + 2] = buffer[index + 2];
            index += 3;

            source_x += 1;

            if source_x >= width {
                source_x  = 0;
                source_y += 1;
            }

        }

        Texture::new(width as u32, height as u32, rgb_image)
    }
}

/// A Sprite is a sub-area on a Texture. Primarily used for Font Bitmaps and GUI-elements.
/// Textures have their origin at the bottom left, such that Sprites are positioned accordingly.
#[allow(dead_code)]
#[derive(Debug)]
pub struct Sprite {

    x      : u32,
    y      : u32,

    width  : u32,
    height : u32,

}

#[allow(dead_code)]
impl Sprite {

    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Sprite {
        Sprite {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }


    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn x_as_f32(&self) -> f32 {
        self.x as f32
    }

    pub fn y_as_f32(&self) -> f32 {
        self.y as f32
    }

    pub fn width_as_f32(&self) -> f32 {
        self.width as f32
    }

    pub fn height_as_f32(&self) -> f32 {
        self.height as f32
    }
}

#[allow(dead_code)]
pub struct Font {
    tex : Texture,
    map : HashMap<char, Sprite>,
}

#[allow(dead_code)]
impl Font {

    pub fn new(tex: Texture, fnt_file: &str) -> Font {
        Font {
            tex : tex,
            map : Font::load_fnt_file(fnt_file),
        }
    }

    pub fn mesh(&self, text:&str) -> mesh::Mesh {
        let mut x : f32 = 0.0;
        let mut y : f32 = 0.0;
        let scale : f32 = 0.005;
        let mut vertex_count = 0;

        let array = AttribArrayBuilder::new()
            .push("position", 3, AttribType::Position)
            .push("uv", 2, AttribType::Uv)
            .build();

        let mut mesh = mesh::Mesh::empty(array);

        for character in text.chars() {

            if character == ' ' {
                let sprite = self.map.get(&'a').unwrap();
                x += sprite.width_as_f32() * scale;
                continue;
            } else if character == '\n' {
                let sprite = self.map.get(&'a').unwrap();
                y -= sprite.height_as_f32() * scale;
                x = 0.0;
                continue;
            }

            let sprite = self.map.get(&character).unwrap();

            let uv_x = sprite.x_as_f32() / self.tex.width_as_f32();
            let uv_y = (self.tex.height()-sprite.y()-sprite.height()) as f32 / self.tex.height_as_f32();

            let uv_width  = sprite.width_as_f32()  / self.tex.width_as_f32();
            let uv_height = sprite.height_as_f32() / self.tex.height_as_f32();

            let height = sprite.height_as_f32() * scale;
            let width  = sprite.width_as_f32()  * scale;

          //  println!("{}, {}, {}", x, uv_x, uv_y);

            mesh.push_vertices( vec![
                vec![x+0.0,   y+0.0,    0.0,  uv_x,            uv_y],
                vec![x+width, y+0.0,    0.0,  uv_x + uv_width, uv_y],
                vec![x+width, y+height, 0.0,  uv_x + uv_width, uv_y + uv_height],
                vec![x+0.0,   y+height, 0.0,  uv_x,            uv_y + uv_height],
            ]);

            x += width;
            vertex_count += 4;

            mesh.push_faces( vec![
                mesh::Face::new(vertex_count - 4, vertex_count - 3, vertex_count - 2),
                mesh::Face::new(vertex_count - 2, vertex_count - 1, vertex_count - 4),
            ]);

        }

        mesh.translate(0.0, -y, 0.0);

        mesh
    }

    pub fn load_fnt_file(filename: &str) -> HashMap<char, Sprite>{
        use std::error::Error;
        use std::fs::File;
        use std::io::prelude::*;
        use std::path::Path;

        // Create a path to the desired file
        let path    = Path::new(filename);
        let display = path.display();

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
            Err(why) => panic!("couldn't open {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut content = String::new();
        match file.read_to_string(&mut content) {
            Err(why) => panic!("couldn't read {}: {}", display, why.description()),
            Ok(_)    => (),//print!("{} contains:\n{}", display, content),
        }

        let mut map : HashMap<char, Sprite> = HashMap::new();

        for line in content.lines() {

            if line.starts_with("char ") {
                let mut id     : u8 = 0;
                let mut x      : u32 = 0;
                let mut y      : u32 = 0;
                let mut width  : u32 = 0;
                let mut height : u32 = 0;

                //println!("line! {}", line);
                let sss : String = line.chars().skip("char ".len()).take(line.len()-"char ".len()).collect();
                //println!("sub line [{}]", sss);

                for s in sss.split_whitespace() {
                    //println!("s as in as [{}]", s);
                    let key_value_pair: Vec<&str> = s.split("=").collect();
                    let key   = key_value_pair[0];
                    let value = key_value_pair[1];
                    //println!("grummel {} -> {}", key, value);
                    match key {
                        "id"     => id     = value.parse::<u8>().unwrap(),
                        "x"      => x      = value.parse::<u32>().unwrap(),
                        "y"      => y      = value.parse::<u32>().unwrap(),
                        "width"  => width  = value.parse::<u32>().unwrap(),
                        "height" => height = value.parse::<u32>().unwrap(),
                        _        => {},
                    }
                }

                if width == 0 {
                    continue;
                }

                //println!("buiding sprite outt of {}, {}, {}, {}, {}", x, y, width, height, id as char);
                let sprite = Sprite::new(x, y, width, height);
                map.insert(id as char, sprite);
            }
        }

        map
    }

    pub fn tex(&self) -> &Texture {
        &self.tex
    }
}
