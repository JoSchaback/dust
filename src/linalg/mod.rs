
use std::f32::consts::PI;

#[allow(dead_code)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[allow(dead_code)]
impl Vector3 {

    pub fn zero() -> Vector3 {
        Vector3 {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 {x: x, y: y, z: z}
    }

    pub fn from_vec3(vec: &Vector3) -> Vector3 {
        Vector3::new(vec.x, vec.y, vec.z)
    }

    pub fn normalize(&mut self) -> &mut Vector3 {

        let d = ( self.x*self.x + self.y*self.y + self.z*self.z ) .sqrt();

        self.x /= d;
        self.y /= d;
        self.z /= d;

        self
    }

    pub fn set(&mut self, xp:f32, yp:f32, zp:f32) {
        self.x = xp;
        self.y = yp;
        self.z = zp;
    }

    pub fn copy(&mut self, vec:&Vector3) {
        self.x = vec.x;
        self.y = vec.y;
        self.z = vec.z;
    }

    pub fn sub(&mut self, xp:f32, yp:f32, zp:f32) {
        self.x -= xp;
        self.y -= yp;
        self.z -= zp;
    }

    pub fn sub_vec(&mut self, vec:&Vector3) {
        self.x -= vec.x;
        self.y -= vec.y;
        self.z -= vec.z;
    }

    pub fn add(&mut self, xp:f32, yp:f32, zp:f32) {
        self.x += xp;
        self.y += yp;
        self.z += zp;
    }

    pub fn cross(&mut self, a:&Vector3) {
        let temp_x = self.y * a.z - self.z * a.y;
        let temp_y = self.z * a.x - self.x * a.z;
        let temp_z = self.x * a.y - self.y * a.x;

        self.x = temp_x;
        self.y = temp_y;
        self.z = temp_z;
    }
}

pub const ZERO : &'static Vector3 = &Vector3 {x : 0.0, y: 0.0, z: 0.0};
pub const Z_UP : &'static Vector3 = &Vector3 {x : 0.0, y: 0.0, z: 1.0};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}

impl Vector4 {

    pub fn zero() -> Vector4 {
        Vector4 {x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }

    pub fn new(x: f32, y: f32, z: f32, w:f32) -> Vector4 {
        Vector4 {x: x, y: y, z: z, w:w}
    }

    pub fn from_vec4(vec: &Vector4) -> Vector4 {
        Vector4::new(vec.x, vec.y, vec.z, vec.w)
    }

    pub fn normalize(&mut self) -> &mut Vector4 {

        let d = ( self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w) .sqrt();

        self.x /= d;
        self.y /= d;
        self.z /= d;
        self.w /= d;

        self
    }

    pub fn set(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
        self.x = xp;
        self.y = yp;
        self.z = zp;
        self.w = wp;
    }

    // TODO this should be rather clone_from()?
    pub fn copy(&mut self, vec:&Vector4) {
        self.x = vec.x;
        self.y = vec.y;
        self.z = vec.z;
        self.w = vec.w;
    }

    pub fn scale_scalar(&mut self, s:f32) {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
    }

    pub fn sub(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
        self.x -= xp;
        self.y -= yp;
        self.z -= zp;
        self.w -= wp;
    }

    pub fn sub_vec(&mut self, vec:&Vector4) {
        self.x -= vec.x;
        self.y -= vec.y;
        self.z -= vec.z;
        self.w -= vec.w;
    }

    pub fn add(&mut self, xp:f32, yp:f32, zp:f32, wp:f32) {
        self.x += xp;
        self.y += yp;
        self.z += zp;
        self.w += wp;
    }

}

#[allow(dead_code)]
pub struct Matrix3 {
    m_0_0 : f32,
    m_0_1 : f32,
    m_0_2 : f32,

    m_1_0 : f32,
    m_1_1 : f32,
    m_1_2 : f32,

    m_2_0 : f32,
    m_2_1 : f32,
    m_2_2 : f32,
}

impl Matrix3 {

    pub fn new() -> Matrix3 {
        Matrix3 {
            m_0_0: 1.0,
            m_1_0: 0.0,
            m_2_0: 0.0,

            m_0_1: 0.0,
            m_1_1: 1.0,
            m_2_1: 0.0,

            m_0_2: 0.0,
            m_1_2: 0.0,
            m_2_2: 1.0,
        }
    }

    pub fn calc_normal_matrix(&mut self, view: &Matrix4) {
        self.m_0_0 = view.m_0_0;
        self.m_1_0 = view.m_1_0;
        self.m_2_0 = view.m_2_0;

        self.m_0_1 = view.m_0_1;
        self.m_1_1 = view.m_1_1;
        self.m_2_1 = view.m_2_1;

        self.m_0_2 = view.m_0_2;
        self.m_1_2 = view.m_1_2;
        self.m_2_2 = view.m_2_2;

        self.inverse();
        self.transpose();
    }

    pub fn inverse(&mut self) {
        let a = self.m_0_0;
        let b = self.m_1_0;
        let c = self.m_2_0;
        let d = self.m_0_1;
        let e = self.m_1_1;
        let f = self.m_2_1;
        let g = self.m_0_2;
        let h = self.m_1_2;
        let i = self.m_2_2;

        let det = a * (e * i - f * h) - b * (i * d - f * g) + c * (d * h - e * g);

        self.m_0_0 = (e * i - f * h) / det;    // A
        self.m_1_0 = -(b * i - c * h) / det;    // D
        self.m_2_0 = (b * f - c * e) / det;    // G
        self.m_0_1 = -(d * i - f * g) / det;    // B
        self.m_1_1 = (a * i - c * g) / det;    // E
        self.m_2_1 = -(a * f - c * d) / det;    // H
        self.m_0_2 = (d * h - e * g) / det;    // C
        self.m_1_2 = -(a * h - b * g) / det;    // F
        self.m_2_2 = (a * e - b * d) / det;    // I
    }

    pub fn transpose(&mut self) {
        let mut tmp;

        tmp        = self.m_0_1;
        self.m_0_1 = self.m_1_0;
        self.m_1_0 = tmp;

        tmp        = self.m_0_2;
        self.m_0_2 = self.m_2_0;
        self.m_2_0 = tmp;

        tmp        = self.m_1_2;
        self.m_1_2 = self.m_2_1;
        self.m_2_1 = tmp;
    }

    pub fn as_ptr(&self) -> *const f32 {
        &self.m_0_0
    }
    /*
    private Matrix3 calcNormalMatrix(Matrix4 mv) {
        // set upper left
        normal.m_0_0 = mv.m_0_0;
        normal.m_1_0 = mv.m_1_0;
        normal.m_2_0 = mv.m_2_0;

        normal.m_0_1 = mv.m_0_1;
        normal.m_1_1 = mv.m_1_1;
        normal.m_2_1 = mv.m_2_1;

        normal.m_0_2 = mv.m_0_2;
        normal.m_1_2 = mv.m_1_2;
        normal.m_2_2 = mv.m_2_2;

        normal.inverse();
        normal.transpose();

        return normal;
    }
    */

}

#[allow(dead_code)]
impl Matrix4 {

    pub fn mult_to_vec4(&self, vec: &mut Vector4) {
        let nx = vec.x * self.m_0_0 + vec.y * self.m_1_0 + vec.z * self.m_2_0 + vec.w * self.m_3_0;
        let ny = vec.x * self.m_0_1 + vec.y * self.m_1_1 + vec.z * self.m_2_1 + vec.w * self.m_3_1;
        let nz = vec.x * self.m_0_2 + vec.y * self.m_1_2 + vec.z * self.m_2_2 + vec.w * self.m_3_2;
        let nw = vec.x * self.m_0_3 + vec.y * self.m_1_3 + vec.z * self.m_2_3 + vec.w * self.m_3_3;

        vec.x = nx;
        vec.y = ny;
        vec.z = nz;
        vec.w = nw;
    }

}

#[allow(dead_code)]
pub struct Matrix4 {
    m_0_0 : f32,
    m_0_1 : f32,
    m_0_2 : f32,
    m_0_3 : f32,

    m_1_0 : f32,
    m_1_1 : f32,
    m_1_2 : f32,
    m_1_3 : f32,

    m_2_0 : f32,
    m_2_1 : f32,
    m_2_2 : f32,
    m_2_3 : f32,

    m_3_0 : f32,
    m_3_1 : f32,
    m_3_2 : f32,
    m_3_3 : f32
}

//trait Matrix4 : Sized {}

#[allow(dead_code)]
impl Matrix4 {

    pub fn as_ptr(&self) -> *const f32 {
        &self.m_0_0
    }

    pub fn rotation(&mut self, alpha: f32, u : &Vector3) {

        self.identity(); // TODO necessary? I think all values get overriden anyways later

        let c = f32::cos(alpha);
        let s = f32::sin(alpha);
        let t = 1.0 - c;

        self.m_0_0 = t * u.x * u.x + c;
        self.m_1_0 = t * u.x * u.y - u.z * s;
        self.m_2_0 = u.x * u.z * t + u.y * s;
        self.m_3_0 = 0.0;
        self.m_0_1 = t * u.y * u.x + u.z * s;
        self.m_1_1 = t * u.y * u.y + c;
        self.m_2_1 = u.y * u.z * t - u.x * s;
        self.m_3_1 = 0.0;
        self.m_0_2 = t * u.z * u.x - u.y * s;
        self.m_1_2 = t * u.z * u.y + u.x * s;
        self.m_2_2 = u.z * u.z * t + c;
        self.m_3_2 = 0.0;
        self.m_0_3 = 0.0;
    }

    pub fn new() -> Matrix4 {
        Matrix4 {
            m_0_0: 1.0,
            m_1_0: 0.0,
            m_2_0: 0.0,
            m_3_0: 0.0,

            m_0_1: 0.0,
            m_1_1: 1.0,
            m_2_1: 0.0,
            m_3_1: 0.0,

            m_0_2: 0.0,
            m_1_2: 0.0,
            m_2_2: 1.0,
            m_3_2: 0.0,

            m_0_3: 0.0,
            m_1_3: 0.0,
            m_2_3: 0.0,
            m_3_3: 1.0,
        }
    }

    pub fn identity(&mut self) {
        self.m_0_0 = 1.0;
        self.m_1_0 = 0.0;
        self.m_2_0 = 0.0;
        self.m_3_0 = 0.0;

        self.m_0_1 = 0.0;
        self.m_1_1 = 1.0;
        self.m_2_1 = 0.0;
        self.m_3_1 = 0.0;

        self.m_0_2 = 0.0;
        self.m_1_2 = 0.0;
        self.m_2_2 = 1.0;
        self.m_3_2 = 0.0;

        self.m_0_3 = 0.0;
        self.m_1_3 = 0.0;
        self.m_2_3 = 0.0;
        self.m_3_3 = 1.0;
    }

    pub fn frustum(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        // http://www.songho.ca/opengl/gl_projectionmatrix.html

        self.identity();

        self.m_0_0 = (2.0 * near) / (right - left);
        self.m_2_0 = (right + left) / (right - left);

        self.m_1_1 = (2.0 * near) / (top - bottom);
        self.m_2_1 = (top + bottom) / (top - bottom);

        self.m_2_2 = -(far + near) / (far - near);
        self.m_3_2 = -2.0 * (far * near) / (far - near);

        self.m_2_3 = -1.0;
        self.m_3_3 = 0.0;
    }

    pub fn copy(&mut self, c:&Matrix4) {
        self.m_0_0 = c.m_0_0;
        self.m_1_0 = c.m_1_0;
        self.m_2_0 = c.m_2_0;
        self.m_3_0 = c.m_3_0;

        self.m_0_1 = c.m_0_1;
        self.m_1_1 = c.m_1_1;
        self.m_2_1 = c.m_2_1;
        self.m_3_1 = c.m_3_1;

        self.m_0_2 = c.m_0_2;
        self.m_1_2 = c.m_1_2;
        self.m_2_2 = c.m_2_2;
        self.m_3_2 = c.m_3_2;

        self.m_0_3 = c.m_0_3;
        self.m_1_3 = c.m_1_3;
        self.m_2_3 = c.m_2_3;
        self.m_3_3 = c.m_3_3;
    }

    pub fn row(&mut self, row: u8, x: f32, y: f32, z: f32, a: f32) {
        match row {
            0 => {
                self.m_0_0 = x;
                self.m_1_0 = y;
                self.m_2_0 = z;
                self.m_3_0 = a;
            },
            1 => {
                self.m_0_1 = x;
                self.m_1_1 = y;
                self.m_2_1 = z;
                self.m_3_1 = a;
            },
            2 => {
                self.m_0_2 = x;
                self.m_1_2 = y;
                self.m_2_2 = z;
                self.m_3_2 = a;
            },
            3 => {
                self.m_0_3 = x;
                self.m_1_3 = y;
                self.m_2_3 = z;
                self.m_3_3 = a;
            },
            _ => panic!("Matrix4x4 has rows 0 to 3, not {}", row),
        };
    }

    pub fn projection(&mut self, view_angle: f32, width: f32, height: f32, near_clipping_plane: f32, far_clipping_plane: f32) {
        // http://www.geeks3d.com/20090729/howto-perspective-projection-matrix-in-opengl/
        let radians: f32 = view_angle * PI / 180.0;
        let half_height = f32::tan(radians / 2.0) * near_clipping_plane;
        let half_scaled_aspect_ratio = half_height * (width / height);
        self.frustum(-half_scaled_aspect_ratio, half_scaled_aspect_ratio, -half_height, half_height, near_clipping_plane, far_clipping_plane);
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.m_0_0 = x;
        self.m_1_0 = 0.0;
        self.m_2_0 = 0.0;
        self.m_3_0 = 0.0;

        self.m_0_1 = 0.0;
        self.m_1_1 = y;
        self.m_2_1 = 0.0;
        self.m_3_1 = 0.0;

        self.m_0_2 = 0.0;
        self.m_1_2 = 0.0;
        self.m_2_2 = z;
        self.m_3_2 = 0.0;

        self.m_0_3 = 0.0;
        self.m_1_3 = 0.0;
        self.m_2_3 = 0.0;
        self.m_3_3 = 1.0;
    }

    pub fn translation(&mut self, x: f32, y: f32, z: f32) {
        self.identity();

        // fourth column
        self.m_3_0 = x;
        self.m_3_1 = y;
        self.m_3_2 = z;
        self.m_3_3 = 1.0;
    }

    pub fn look_at(&mut self, eye: &Vector3, center: &Vector3, up: &Vector3) {
        let mut u = Vector3::zero();
        let mut v = Vector3::zero();
        let mut w = Vector3::from_vec3(eye);

        // the w vector is computed by w = eye - center which means
        // it is the inverse of the viewing direction.
        w.copy(&eye);
        w.sub_vec(&center);
        w.normalize();

        // compute cross product
        u.copy(&up);
        u.cross(&w);
        u.normalize();
        // side = (0,0,1) x w

        // up = side x look
        v.copy(&w);
        v.cross(&u);
        v.normalize();
        ////v.set(w).cross(u).normalize();

        self.identity();

        self.row(0, u.x, u.y, u.z, 0.0);
        self.row(1, v.x, v.y, v.z, 0.0);
        self.row(2, w.x, w.y, w.z, 0.0);

        let mut trans = Matrix4::new();
        trans.m_3_0 = -eye.x;
        trans.m_3_1 = -eye.y;
        trans.m_3_2 = -eye.z;

        self.mult(&trans);
    }

    pub fn mult(&mut self, that: &Matrix4) {
        let m00 = self.m_0_0 * that.m_0_0 + self.m_1_0 * that.m_0_1 + self.m_2_0 * that.m_0_2 + self.m_3_0 * that.m_0_3;
        let m01 = self.m_0_1 * that.m_0_0 + self.m_1_1 * that.m_0_1 + self.m_2_1 * that.m_0_2 + self.m_3_1 * that.m_0_3;
        let m02 = self.m_0_2 * that.m_0_0 + self.m_1_2 * that.m_0_1 + self.m_2_2 * that.m_0_2 + self.m_3_2 * that.m_0_3;
        let m03 = self.m_0_3 * that.m_0_0 + self.m_1_3 * that.m_0_1 + self.m_2_3 * that.m_0_2 + self.m_3_3 * that.m_0_3;

        let m10 = self.m_0_0 * that.m_1_0 + self.m_1_0 * that.m_1_1 + self.m_2_0 * that.m_1_2 + self.m_3_0 * that.m_1_3;
        let m11 = self.m_0_1 * that.m_1_0 + self.m_1_1 * that.m_1_1 + self.m_2_1 * that.m_1_2 + self.m_3_1 * that.m_1_3;
        let m12 = self.m_0_2 * that.m_1_0 + self.m_1_2 * that.m_1_1 + self.m_2_2 * that.m_1_2 + self.m_3_2 * that.m_1_3;
        let m13 = self.m_0_3 * that.m_1_0 + self.m_1_3 * that.m_1_1 + self.m_2_3 * that.m_1_2 + self.m_3_3 * that.m_1_3;

        let m20 = self.m_0_0 * that.m_2_0 + self.m_1_0 * that.m_2_1 + self.m_2_0 * that.m_2_2 + self.m_3_0 * that.m_2_3;
        let m21 = self.m_0_1 * that.m_2_0 + self.m_1_1 * that.m_2_1 + self.m_2_1 * that.m_2_2 + self.m_3_1 * that.m_2_3;
        let m22 = self.m_0_2 * that.m_2_0 + self.m_1_2 * that.m_2_1 + self.m_2_2 * that.m_2_2 + self.m_3_2 * that.m_2_3;
        let m23 = self.m_0_3 * that.m_2_0 + self.m_1_3 * that.m_2_1 + self.m_2_3 * that.m_2_2 + self.m_3_3 * that.m_2_3;

        let m30 = self.m_0_0 * that.m_3_0 + self.m_1_0 * that.m_3_1 + self.m_2_0 * that.m_3_2 + self.m_3_0 * that.m_3_3;
        let m31 = self.m_0_1 * that.m_3_0 + self.m_1_1 * that.m_3_1 + self.m_2_1 * that.m_3_2 + self.m_3_1 * that.m_3_3;
        let m32 = self.m_0_2 * that.m_3_0 + self.m_1_2 * that.m_3_1 + self.m_2_2 * that.m_3_2 + self.m_3_2 * that.m_3_3;
        let m33 = self.m_0_3 * that.m_3_0 + self.m_1_3 * that.m_3_1 + self.m_2_3 * that.m_3_2 + self.m_3_3 * that.m_3_3;

        self.m_0_0 = m00;
        self.m_0_1 = m01;
        self.m_0_2 = m02;
        self.m_0_3 = m03;

        self.m_1_0 = m10;
        self.m_1_1 = m11;
        self.m_1_2 = m12;
        self.m_1_3 = m13;

        self.m_2_0 = m20;
        self.m_2_1 = m21;
        self.m_2_2 = m22;
        self.m_2_3 = m23;

        self.m_3_0 = m30;
        self.m_3_1 = m31;
        self.m_3_2 = m32;
        self.m_3_3 = m33;
    }
    //public Matrix4 multAssign(Matrix4 that) {
}