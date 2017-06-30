extern crate gl;

use super::{AttribArray, VertexBufferObject, AttribType};
use std::mem;
use std;

/// Datastructure to store a triangulated surface of Vertices and Faces (triangles).
#[allow(dead_code)]
pub struct Mesh {
    vertices        : Vec<Vec<f32>>,
    faces           : Vec<Face>,
    attribs         : AttribArray,
}

#[allow(dead_code)]
impl Mesh {

    /// Returns a new Mesh, initialized with the given Vertices and Faces.
    pub fn new(vertices: Vec<Vec<f32>>, faces: Vec<Face>, attribs: AttribArray) -> Mesh {
        // TODO add check that Vertices have same and correct length

        Mesh {
            vertices: vertices,
            faces   : faces,
            attribs : attribs,
        }
    }

    pub fn empty(attribs: AttribArray) -> Mesh {
        Mesh {
            vertices: Vec::new(),
            faces   : Vec::new(),
            attribs : attribs
        }
    }

    pub fn attribs(&self) -> &AttribArray {
        &self.attribs
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        let offset = self.attribs.by_type(AttribType::Position).unwrap().offset;

        for vertex in &mut self.vertices {
            vertex[offset + 0] += x;
            vertex[offset + 1] += y;
            vertex[offset + 2] += z;
        }
    }

    pub fn push_vertex(&mut self, vec: Vec<f32>) {
        // TODO check length
        self.vertices.push(vec);
    }

    pub fn push_vertices(&mut self, vecs: Vec<Vec<f32>>) {
        // TODO check length of vertices

        for vertex in vecs {
            self.vertices.push(vertex);
        }
    }

    pub fn push_faces(&mut self, vecs: Vec<Face>) {
        // TODO check integrity

        for face in vecs {
            self.faces.push(face);
        }
    }

    pub fn push_face(&mut self, face: Face) {
        // TODO check integrity (ie. indices)
        self.faces.push(face);
    }

    pub fn faces(&self) -> &Vec<Face> {
        &self.faces
    }

    pub fn vertex(&self, index: usize) -> &Vec<f32> {
        &self.vertices[index]
    }

    pub fn vertex_mut(&mut self, index: usize) -> &mut Vec<f32> {
        &mut self.vertices[index]
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn vertices(&self) -> &Vec<Vec<f32>> {
        &self.vertices
    }

    pub fn apply_to_3<F>(&mut self, offset: usize, f: F)
        where F: Fn(f32, f32, f32) -> (f32, f32, f32) {
        for vertex in &mut self.vertices {
            let triple = f( vertex[offset], vertex[offset+1], vertex[offset+2] );
            vertex[offset + 0] = triple.0;
            vertex[offset + 1] = triple.1;
            vertex[offset + 2] = triple.2;
        }
    }

    pub fn apply_on_vertices<F>(&mut self, f: F)
        where F: Fn(&mut Vec<f32>) {
        for vertex in &mut self.vertices {
            f( vertex );
            //println!("vertex {:?}", vertex);
        }
    }


    pub fn normalize(&mut self, attrib_name: &str) {
        let offset = self.attribs.by_name(attrib_name).unwrap().offset;
        for vertex in &mut self.vertices {
            let scale = ( vertex[offset]*vertex[offset] + vertex[offset+1]*vertex[offset+1] + vertex[offset+2]*vertex[offset+2] ).sqrt();
            vertex[offset+0] /= scale;
            vertex[offset+1] /= scale;
            vertex[offset+2] /= scale;
        }
    }

    /// Constructs a new VertexBufferObject by writing the Vertex information directly into
    /// a newly allocated graphics card memory region.
    pub fn to_array_buffer_vbo(&self) -> VertexBufferObject {

        let vertex_count = self.faces.len() * 3;
        let stride       = self.vertices[0].len() as usize;
        unsafe {
            let mut vb = mem::uninitialized();

            gl::GenBuffers(1, &mut vb);
            gl::BindBuffer(gl::ARRAY_BUFFER, vb);

            let buffer_size = vertex_count * stride * mem::size_of::<f32>();

            gl::BufferData(gl::ARRAY_BUFFER, buffer_size as gl::types::GLsizeiptr, std::ptr::null() as *const _, gl::STATIC_DRAW);
            let ptr = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut f32;

            let mut offset = 0 as isize;

            for face in &self.faces {
                std::ptr::copy_nonoverlapping(self.vertices[face.v1].as_ptr(), ptr.offset(offset), stride);
                offset += stride as isize;

                std::ptr::copy_nonoverlapping(self.vertices[face.v2].as_ptr(), ptr.offset(offset), stride);
                offset += stride as isize;

                std::ptr::copy_nonoverlapping(self.vertices[face.v3].as_ptr(), ptr.offset(offset), stride);
                offset += stride as isize;

            }

            gl::UnmapBuffer(gl::ARRAY_BUFFER);

            VertexBufferObject::new(vb, None, 0, vertex_count as i32)
        }
    }

    pub fn to_element_array_buffer_vbo(&self) -> VertexBufferObject {

        let vertex_count = self.vertex_count();
        let stride       = self.vertex(0).len() as usize;

        // TODO I realize that Mesh already holds the data in the correct alignment. We could just upload it plainly, without iterating over it.
        // BUT that would require that we check and fix the capacity of the Vertex vecs....

        unsafe {
            let mut array_buffer_id = mem::uninitialized();

            gl::GenBuffers(1, &mut array_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, array_buffer_id);

            let buffer_size = vertex_count * stride * mem::size_of::<f32>();

            gl::BufferData(gl::ARRAY_BUFFER, buffer_size as gl::types::GLsizeiptr, std::ptr::null() as *const _, gl::STATIC_DRAW);
            let ptr = gl::MapBuffer(gl::ARRAY_BUFFER, gl::WRITE_ONLY) as *mut f32;

            //println!("ptr {:?}, vertex_count {}, buffer_size {}, mesh.attribs.stride {}", ptr, vertex_count, buffer_size, stride);

            let mut offset = 0 as isize;

            for vertex in self.vertices() {
                //println!("writing vertex {} ", offset);

                std::ptr::copy_nonoverlapping(vertex.as_ptr(), ptr.offset(offset), stride);
                offset += stride as isize;
            }

            gl::UnmapBuffer(gl::ARRAY_BUFFER);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            //array_buffer = VertexBufferObject2::new();

            let mut element_array_buffer_id = mem::uninitialized();

            gl::GenBuffers(1, &mut element_array_buffer_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_array_buffer_id);

            let element_buffer_size = self.faces.len() * 3 * mem::size_of::<u16>();
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, element_buffer_size as gl::types::GLsizeiptr, std::ptr::null() as *const _, gl::STATIC_DRAW);
            let ptr = gl::MapBuffer(gl::ELEMENT_ARRAY_BUFFER, gl::WRITE_ONLY) as *mut u16;
            //println!("ptr {:?}, mesh.faces.len() {}, buffer_size {}", ptr, self.faces().len(), element_buffer_size);
            let mut offset = 0 as isize;
            for face in self.faces() {
                //println!("writing face index {} ", offset);

                std::ptr::write::<u16>(ptr.offset(offset), face.v1() as u16);
                offset += 1 as isize;
                std::ptr::write::<u16>(ptr.offset(offset), face.v2() as u16);
                offset += 1 as isize;
                std::ptr::write::<u16>(ptr.offset(offset), face.v3() as u16);
                offset += 1 as isize;
            }

            gl::UnmapBuffer(gl::ELEMENT_ARRAY_BUFFER);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            VertexBufferObject::new(array_buffer_id, Some(element_array_buffer_id), element_buffer_size as i32, stride as i32)
        }
    }

}


#[allow(dead_code)]
/// A Triangle face used to shade the area between three Vertices.
pub struct Face {
    v1: usize,
    v2: usize,
    v3: usize,
}

#[allow(dead_code)]
impl Face {

    /// Creates a new Face
    pub fn new(v1: usize, v2: usize, v3: usize) -> Face {
        Face {
            v1: v1,
            v2: v2,
            v3: v3,
        }
    }

    pub fn v1(&self) -> usize {
        self.v1
    }

    pub fn v2(&self) -> usize {
        self.v2
    }

    pub fn v3(&self) -> usize {
        self.v3
    }
}

