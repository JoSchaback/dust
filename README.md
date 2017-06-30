# Dust
Hi, this is Dust, a light-weight tool set for OpenGL development in Rust. I hope you enjoy it.

I (Johannes Schaback) ported it from Java where I have been using a similar library for many years to support me in
developing OpenGL applications for many years. It consists currently of two components:

1. OpenGL: 
Dust comes with several wrapper structs to easy your live when dealing with various OpenGL thingies, such as
VertexBufferObjects (VBO), Textures and VertexAttribs.
In addition, Dust comes with a Mesh struct that helps you on dealing with 3d geometry and to convert it such that
you shader can digest it.

2. LinAlg: 
Dust has a built-in, super simple and light-weight linear algebra module that provides fundamental concepts such as
Vectors and Matrices. They are built for speed and easy of use.

## Examples
There are several examples that may be worth checking out

### Colored Cube
Simple example showing a cube with differently colored faces. Run with `cargo run --example colored_cube`.

![Colored Cube](/assets/colored_cube.JPG)

### Textured Cube
Simple example showing a textured cube. Run with `cargo run --example textured_cube`.

![Textured Cube](/assets/textured_cube.JPG)

### Lighted Sphere
Simple sphere lit by a single light source. Run with `cargo run --example lighted_sphere`.

![Lighted Sphere](/assets/lighted_sphere.JPG)
