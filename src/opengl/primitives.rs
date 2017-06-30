
use opengl::{AttribArrayBuilder, AttribType};
use opengl::mesh::{Mesh, Face};

#[allow(dead_code)]
pub fn cube() -> Mesh {

    let array = AttribArrayBuilder::new()
        .push("position", 3, AttribType::Position)
        .push("normal", 3, AttribType::Normal)
        .push("color", 3, AttribType::ColorRgb)
        .push("uv", 2, AttribType::Uv)
        .build();

    // VertexBuilder::new().push3i(0, 0, 0).push3i(0, 0, 0).push3i(0, 0, 0).push2i(0, 0).build();

    let int_vertices = vec![
        vec![0, 0, 0,  0, 0, -1,  1, 0, 0,  0, 0],
        vec![0, 0, 0,  0, 0, -1,  1, 0, 0,  0, 0],
        vec![0, 0, 0,  0, 0, -1,  1, 0, 0,  0, 0],
        vec![1, 0, 0,  0, 0, -1,  1, 0, 0,  0, 1],

        vec![0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 0],
        vec![1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1],
        vec![1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1],
        vec![0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0],

        vec![0, 0, 0, -1, 0, 0, 0, 1, 1, 0, 0],
        vec![0, 0, 1, -1, 0, 0, 0, 1, 1, 1, 0],
        vec![0, 1, 1, -1, 0, 0, 0, 1, 1, 1, 1],
        vec![0, 1, 0, -1, 0, 0, 0, 1, 1, 0, 1],

        vec![1, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0],
        vec![1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1],
        vec![1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1],
        vec![1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 0],

        vec![0, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0],
        vec![1, 0, 0, 0, -1, 0, 0, 0, 1, 0, 1],
        vec![1, 0, 1, 0, -1, 0, 0, 0, 1, 1, 1],
        vec![0, 0, 1, 0, -1, 0, 0, 0, 1, 1, 0],

        vec![0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0],
        vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1],
        vec![1, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
        vec![0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0],
    ];

    let float_vertices = convert_to_float_vecs(int_vertices);

    let mut faces = Vec::<Face>::new();

    let mut i = 0 as usize;

    while i < float_vertices.len() {

        faces.push( Face::new(i + 0, i + 1, i + 2) );
        faces.push( Face::new(i + 2, i + 3, i + 0) );

        i += 4;
    }

    let mut mesh = Mesh::new(float_vertices, faces, array);

    &mesh.translate(-0.5, -0.5, -0.5);

    // TODO what about translating the mesh to the center?

    mesh
}

fn convert_to_float_vecs(int_vecs: Vec<Vec<i32>>) -> Vec<Vec<f32>> {
    let mut float_vertices = Vec::<Vec<f32>>::with_capacity(int_vecs.len());

    for vec in int_vecs {
        let mut new_vec = Vec::<f32>::with_capacity(vec.len());
        for k in vec {
            new_vec.push(k as f32);
        }
        float_vertices.push(new_vec);
    }

    float_vertices
}

#[allow(dead_code)]
pub fn quad() -> Mesh {
    let array = AttribArrayBuilder::new()
        .push("position", 3, AttribType::Position)
        .push("normal", 3, AttribType::Normal)
        .push("color", 3, AttribType::ColorRgb)
        .push("uv", 2, AttribType::Uv)
        .build();

    let vertices3 = vec![
        vec![0, 0, 0,  0, 0, -1,  1, 0, 0,  0, 0],
        vec![0, 0, 0,  0, 0, -1,  1, 0, 0,  0, 0],
        vec![0, 0, 0,  0, 0, -1,  1, 0, 0,  0, 0],
        vec![1, 0, 0,  0, 0, -1,  1, 0, 0,  0, 1],
    ];

    let float_vertices = convert_to_float_vecs(vertices3);

    let mut faces = Vec::<Face>::new();
    faces.push( Face::new(0, 1, 2) );
    faces.push( Face::new(0, 3, 2) );

    let mut mesh = Mesh::new(float_vertices, faces, array);

    mesh.translate(-0.5, -0.5, -0.5);

    mesh
}

pub fn icosphere(subdivides: u8) -> Mesh {
    let array = AttribArrayBuilder::new()
        .push("position", 3, AttribType::Position)
        .push("normal", 3, AttribType::Normal)
        .push("color", 3, AttribType::ColorRgb)
        .push("uv", 2, AttribType::Uv)
        .build();

    let t = (1.0 + (5.0 as f32).sqrt() ) / 2.0;

    // http://blog.andreaskahler.com/2009/06/creating-icosphere-mesh-in-code.html
    let mut mesh = Mesh::empty(array);

    mesh.push_vertices( vec![
        vec![-1.0,  t, 0.0,  0.0, 0.0, 0.0,  1.0, 0.0, 0.0,  0.0, 0.0],
        vec![ 1.0,  t, 0.0,  0.0, 0.0, 0.0,  0.0, 0.0, 1.0,  0.0, 0.0],
        vec![-1.0, -t, 0.0,  0.0, 0.0, 0.0,  1.0, 1.0, 0.0,  0.0, 0.0],
        vec![ 1.0, -t, 0.0,  0.0, 0.0, 0.0,  0.0, 1.0, 0.0,  0.0, 0.0],

        vec![0.0, -1.0,  t,  0.0, 0.0, 0.0,  1.0, 0.0, 0.0,  0.0, 0.0],
        vec![0.0,  1.0,  t,  0.0, 0.0, 0.0,  0.0, 1.0, 1.0,  0.0, 0.0],
        vec![0.0, -1.0, -t,  0.0, 0.0, 0.0,  0.0, 1.0, 0.0,  0.0, 0.0],
        vec![0.0,  1.0, -t,  0.0, 0.0, 0.0,  0.0, 1.0, 1.0,  0.0, 0.0],

        vec![ t, 0.0, -1.0,  0.0, 0.0, 0.0,  1.0, 0.0, 1.0,  0.0, 0.0],
        vec![ t, 0.0,  1.0,  0.0, 0.0, 0.0,  0.0, 0.0, 0.0,  0.0, 0.0],
        vec![-t, 0.0, -1.0,  0.0, 0.0, 0.0,  1.0, 0.0, 0.0,  0.0, 0.0],
        vec![-t, 0.0,  1.0,  0.0, 0.0, 0.0,  0.0, 1.0, 0.0,  0.0, 0.0],
    ]);

    mesh.push_faces( vec![
        Face::new(0, 11,  5),
        Face::new(0,  5,  1),
        Face::new(0,  1,  7),
        Face::new(0,  7, 10),
        Face::new(0, 10, 11),

        Face::new(1,   5, 9),
        Face::new(5,  11, 4),
        Face::new(11, 10, 2),
        Face::new(10,  7, 6),
        Face::new(7,   1, 8),

        Face::new(3, 9, 4),
        Face::new(3, 4, 2),
        Face::new(3, 2, 6),
        Face::new(3, 6, 8),
        Face::new(3, 8, 9),

        Face::new(4, 9,  5),
        Face::new(2, 4, 11),
        Face::new(6, 2, 10),
        Face::new(8, 6,  7),
        Face::new(9, 8,  1),
    ]);

    mesh.normalize( "position" );
    mesh.normalize( "normal" );

    // subdivide the faces such that the surface of the sphere becomes more smooth
    for _ in 0..subdivides {
        mesh = subdivide(mesh);
    }

    // fix the Normals. So far, we have not computed any Normal data.
    // Under normal circumstances, I would first do

    //let position_offset = &mesh.attribs().attrib_by_type(AttribType::Position);
    //let normal_offset   = &mesh.attribs().attrib_by_type(AttribType::Normal);

    // to get the offsets in the Vertex vector, but since I ca not borrow immutably
    // from mesh unless I invalide the reference, I hard code the offsets.
    let position_offset : usize = 0;
    let normal_offset   : usize = 3;
    mesh.apply_on_vertices( |vertex: &mut Vec<f32>| {
        vertex[normal_offset] = vertex[position_offset];
        vertex[normal_offset + 1] = vertex[position_offset + 1];
        vertex[normal_offset + 2] = vertex[position_offset + 2];
    });

    //mesh.normalize( "position" );
    //mesh.normalize( "normal" );

    mesh
}

fn subdivide(mesh: Mesh) -> Mesh {
    /*
        // take each face, subdivide it, register the new vertices and new faces and throw the old
        // face out

                 v0
                 /\
                /  \
               /    \
              /------\
             /\      /\
            /  \    /  \
           /    \  /    \
        v1 ------\/------ v2
*/
    //let old_faces = mesh.faces().clone();

    let dimension = mesh.vertex(0).len();

    let mut new_mesh = Mesh::empty(mesh.attribs().clone());

    new_mesh.push_vertices( mesh.vertices().clone() );

    let mut vertex_count = mesh.vertex_count();

    for face in mesh.faces() {
        let mut v0tov1 : Vec<f32> = Vec::with_capacity(dimension);
        let mut v1tov2 : Vec<f32> = Vec::with_capacity(dimension);
        let mut v2tov0 : Vec<f32> = Vec::with_capacity(dimension);

        unsafe {
            v0tov1.set_len(dimension);
            v1tov2.set_len(dimension);
            v2tov0.set_len(dimension);
        }

        let v0 = mesh.vertex(face.v1());
        let v1 = mesh.vertex(face.v2());
        let v2 = mesh.vertex(face.v3());

        for i in 0..dimension {
            v0tov1[i] = (v0[i] + v1[i]) / 2.0;
            v1tov2[i] = (v1[i] + v2[i]) / 2.0;
            v2tov0[i] = (v2[i] + v0[i]) / 2.0;
        }

        let mut scale = (v0tov1[0]*v0tov1[0] + v0tov1[1]*v0tov1[1] + v0tov1[2]*v0tov1[2]).sqrt();
        v0tov1[0] /= scale;
        v0tov1[1] /= scale;
        v0tov1[2] /= scale;

        scale = (v1tov2[0]*v1tov2[0] + v1tov2[1]*v1tov2[1] + v1tov2[2]*v1tov2[2]).sqrt();
        v1tov2[0] /= scale;
        v1tov2[1] /= scale;
        v1tov2[2] /= scale;

        scale = (v2tov0[0]*v2tov0[0] + v2tov0[1]*v2tov0[1] + v2tov0[2]*v2tov0[2]).sqrt();
        v2tov0[0] /= scale;
        v2tov0[1] /= scale;
        v2tov0[2] /= scale;

        new_mesh.push_vertex(v0tov1);
        new_mesh.push_vertex(v1tov2);
        new_mesh.push_vertex(v2tov0);

        vertex_count += 3;

        new_mesh.push_face(Face::new(face.v2(), vertex_count - 2, vertex_count - 3));
        new_mesh.push_face(Face::new(face.v1(), vertex_count - 3, vertex_count - 1));
        new_mesh.push_face(Face::new(vertex_count - 2, face.v3(), vertex_count - 1));
        new_mesh.push_face(Face::new(vertex_count - 3, vertex_count - 2, vertex_count - 1));
    }

    new_mesh
}