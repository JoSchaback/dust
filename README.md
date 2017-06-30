# Dust
Hi, this is Dust, a light-weight tool set for OpenGL development in Rust. I hope you enjoy it.

I (Johannes Schaback) ported it from Java where I have been using a similar library for many years to support me in
developing OpenGL applications for many years. It consists currently of two components:

## OpenGL
Dust comes with several wrapper structs to easy your live when dealing with various OpenGL thingies, such as
VertexBufferObjects (VBO), Textures and VertexAttribs.
In addition, Dust comes with a Mesh struct that helps you on dealing with 3d geometry and to convert it such that
you shader can digest it.

## LinAlg
Dust has a built-in, super simple and light-weight linear algebra module that provides fundamental concepts such as
Vectors and Matrices. They are built for speed and easy of use.



