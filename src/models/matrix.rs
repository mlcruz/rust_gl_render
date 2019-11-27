use glm::abs;
use glm::cos;
use glm::sin;
use glm::sqrt;
use glm::tan;
use glm::Vector4;

#[allow(dead_code)]
#[derive(Copy, Debug)]
pub struct GLMatrix {
    pub matrix: glm::Mat4,
}

// glm::mat4 espera a matrix em row major
pub const fn points_to_mat4(points: &[f32; 16]) -> glm::Mat4 {
    // glm::mat4(
    //     points[0], points[4], points[8], points[12], points[1], points[5], points[9], points[13],
    //     points[2], points[6], points[10], points[14], points[3], points[7], points[11], points[15],
    // )

    glm::Mat4 {
        c0: Vector4 {
            x: points[0],
            y: points[4],
            z: points[8],
            w: points[12],
        },
        c1: Vector4 {
            x: points[1],
            y: points[5],
            z: points[9],
            w: points[13],
        },
        c2: Vector4 {
            x: points[2],
            y: points[6],
            z: points[10],
            w: points[14],
        },
        c3: Vector4 {
            x: points[3],
            y: points[7],
            z: points[11],
            w: points[15],
        },
    }
}

#[allow(dead_code)]
pub const fn identity_matrix() -> GLMatrix {
    GLMatrix::new([
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ])
}

pub const fn translating_matrix(x: f32, y: f32, z: f32) -> GLMatrix {
    GLMatrix::new([
        1.0, 0.0, 0.0, x, // LINHA 1
        0.0, 1.0, 0.0, y, // LINHA 2
        0.0, 0.0, 1.0, z, // LINHA 3
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[allow(dead_code)]
pub const fn scaling_matrix(x: f32, y: f32, z: f32) -> GLMatrix {
    GLMatrix::new([
        x, 0.0, 0.0, 0.0, // LINHA 1
        0.0, y, 0.0, 0.0, // LINHA 2
        0.0, 0.0, z, 0.0, // LINHA 3
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[allow(dead_code)]
pub fn rotation_matrix_x(angle: f32) -> GLMatrix {
    let c = cos(angle);
    let s = sin(angle);
    GLMatrix::new([
        1.0, 0.0, 0.0, 0.0, // LINHA 1
        0.0, c, -s, 0.0, // LINHA 2
        0.0, s, c, 0.0, // LINHA 3
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[allow(dead_code)]
pub fn rotation_matrix_y(angle: f32) -> GLMatrix {
    let c = cos(angle);
    let s = sin(angle);
    GLMatrix::new([
        c, 0.0, s, 0.0, // LINHA 1
        0.0, 1.0, 0.0, 0.0, // LINHA 2
        -s, 0.0, c, 0.0, // LINHA 3
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[allow(dead_code)]
pub fn rotation_matrix_z(angle: f32) -> GLMatrix {
    let c = cos(angle);
    let s = sin(angle);
    GLMatrix::new([
        c, -s, 0.0, 0.0, // LINHA 1
        s, c, 0.0, 0.0, // LINHA 2
        0.0, 0.0, 0.0, 0.0, // LINHA 3
        0.0, 0.0, 0.0, 1.0,
    ])
}

#[allow(dead_code)]
pub fn norm(v: glm::Vec4) -> f32 {
    let vx = v.x;
    let vy = v.y;
    let vz = v.z;
    sqrt(vx * vx + vy * vy + vz * vz)
}

#[allow(dead_code)]
pub fn rotation_matrix(angle: f32, axis: glm::Vec4) -> GLMatrix {
    let c = cos(angle);
    let s = sin(angle);

    let v = axis / norm(axis);
    let vx = v.x;
    let vy = v.y;
    let vz = v.z;
    let mc = 1.0 - c;

    GLMatrix::new([
        (vx * vx) * mc + c, // LINHA 1
        (vx * vy) * mc - vz * s,
        (vx * vz * mc) + vy * s,
        0.0,
        (vx * vy * mc) + vz * s, // LINHA 2
        (vy * vy) * mc + c,
        (vz * vy) * mc - vx * s,
        0.0,
        (vx * vz) * mc - vy * s, // LINHA 3
        (vy * vz) * mc + vx * s,
        (vz * vz) * mc + c,
        0.0,
        0.0, // LINHA 4
        0.0,
        0.0,
        1.0,
    ])
}

#[allow(dead_code)]
pub fn cross_product(u: glm::Vec4, v: glm::Vec4) -> glm::Vec4 {
    let u1 = u.x;
    let u2 = u.y;
    let u3 = u.z;

    let v1 = v.x;
    let v2 = v.y;
    let v3 = v.z;

    glm::Vec4 {
        x: u2 * v3 - u3 * v2, // Primeiro coeficiente
        y: u3 * v1 - u1 * v3, // Segundo coeficiente
        z: u1 * v2 - u2 * v1, // terceiro coeficiente
        w: 0.0,               // w = 0 para vetores.
    }
}

#[allow(dead_code)]
pub fn dot_product(u: glm::Vec4, v: glm::Vec4) -> f32 {
    let u1 = u.x;
    let u2 = u.y;
    let u3 = u.z;
    let u4 = u.w;
    let v1 = v.x;
    let v2 = v.y;
    let v3 = v.z;
    let v4 = v.w;

    if u4 != 0.0 || v4 != 0.0 {
        panic!("ERROR: Produto escalar não definido para pontos.\n");
    }
    return u1 * v1 + u2 * v2 + u3 * v3 + u4 * v4;
}

#[allow(dead_code)]
pub fn ortographic_matrix(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> GLMatrix {
    GLMatrix::new([
        2.0 / (r - l), // LINHA 1
        0.0,
        0.0,
        -((r + l) / (r - l)),
        0.0, // LINHA 2
        2.0 / (t - b),
        0.0,
        -((t + b) / (t - b)),
        0.0, // LINHA 3
        0.0,
        2.0 / (f - n),
        -((f + n) / (f - n)),
        0.0, // LINHA 4
        0.0,
        0.0,
        1.0,
    ])
}

#[allow(dead_code)]
pub fn perpective_matrix(field_of_view: f32, aspect: f32, n: f32, f: f32) -> GLMatrix {
    let t = abs(n) * tan(field_of_view / 2.0);
    let b = -t;
    let r = t * aspect;
    let l = -r;

    let p = GLMatrix::new([
        n,
        0.0,
        0.0,
        0.0, // LINHA 1
        0.0,
        n,
        0.0,
        0.0, // LINHA 2
        0.0,
        0.0,
        n + f,
        -f * n, // LINHA 3
        0.0,
        0.0,
        1.0,
        0.0, // LINHA 4);
    ]);
    let m = ortographic_matrix(l, r, b, t, n, f);
    let mp = (-m.matrix) * p.matrix;
    GLMatrix { matrix: mp }
}

#[allow(dead_code)]
pub fn compute_normal(p1: &glm::Vec4, p2: &glm::Vec4, p3: &glm::Vec4) -> glm::Vec4 {
    let u = *p3 - *p1;
    let v = *p2 - *p1;
    -cross_product(u, v)
}

#[allow(dead_code)]
pub fn normalize_vector(v: glm::Vec4) -> glm::Vec4 {
    // Trata divisão por 0
    if v.x != 0.0 || v.y != 0.0 || v.z != 0.0 || v.w != 0.0 {
        v / norm(v)
    } else {
        v
    }
}
#[allow(dead_code)]
pub fn camera_view_matrix(
    position_c: glm::Vec4,
    view_vector: glm::Vec4,
    up_vector: glm::Vec4,
) -> GLMatrix {
    let mut w = -view_vector;
    let mut u = cross_product(up_vector, w);
    w = normalize_vector(w);
    u = normalize_vector(u);

    let v = cross_product(w, u);
    let origin_o = glm::Vec4 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    let ux = u.x;
    let uy = u.y;
    let uz = u.z;

    let vx = v.x;
    let vy = v.y;
    let vz = v.z;

    let wx = w.x;
    let wy = w.y;
    let wz = w.z;

    let c = position_c - origin_o;

    GLMatrix::new([
        ux, // LINHA 1
        uy,
        uz,
        dot_product(-u, c),
        vx, // LINHA 2
        vy,
        vz,
        dot_product(-v, c),
        wx, // LINHA 3
        wy,
        wz,
        dot_product(-w, c),
        0.0, // LINHA 4
        0.0,
        0.0,
        1.0,
    ])
}

#[allow(dead_code)]
impl GLMatrix {
    pub const fn new(points: [f32; 16]) -> Self {
        GLMatrix {
            matrix: points_to_mat4(&points),
        }
    }

    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        let translation_matrix = translating_matrix(x, y, z);
        GLMatrix {
            matrix: translation_matrix.matrix * self.matrix,
        }
    }

    pub fn rotate_x(&self, angle: f32) -> Self {
        let rotation_matrix = rotation_matrix_x(angle);
        GLMatrix {
            matrix: rotation_matrix.matrix * self.matrix,
        }
    }

    pub fn rotate_y(&self, angle: f32) -> Self {
        let rotation_matrix = rotation_matrix_y(angle);
        GLMatrix {
            matrix: rotation_matrix.matrix * self.matrix,
        }
    }

    pub fn rotate_z(&self, angle: f32) -> Self {
        let rotation_matrix = rotation_matrix_z(angle);
        GLMatrix {
            matrix: rotation_matrix.matrix * self.matrix,
        }
    }

    pub fn rotate(&self, angle: f32, axis: glm::Vec4) -> Self {
        let rotation_matrix = rotation_matrix(angle, axis);
        GLMatrix {
            matrix: rotation_matrix.matrix * self.matrix,
        }
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        let scaling_matrix = scaling_matrix(x, y, z);
        GLMatrix {
            matrix: scaling_matrix.matrix * self.matrix,
        }
    }

    pub fn update(&mut self, matrix: &GLMatrix) {
        self.matrix = matrix.matrix;
    }
}

impl Clone for GLMatrix {
    fn clone(&self) -> Self {
        GLMatrix {
            matrix: self.matrix,
        }
    }
}

impl From<[f32; 16]> for GLMatrix {
    fn from(points: [f32; 16]) -> Self {
        GLMatrix {
            matrix: points_to_mat4(&points),
        }
    }
}

pub trait MatrixTransform: Sized {
    fn get_matrix(&self) -> &GLMatrix;
    fn update_matrix(&mut self, matrix: &GLMatrix) -> &mut Self;
    fn from_matrix(&self, matrix: &GLMatrix) -> Self;

    fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        self.from_matrix(&self.get_matrix().translate(x, y, z))
    }

    fn rotate_x(&self, angle: f32) -> Self {
        self.from_matrix(&self.get_matrix().rotate_x(angle))
    }

    fn rotate_y(&self, angle: f32) -> Self {
        self.from_matrix(&self.get_matrix().rotate_y(angle))
    }

    fn rotate_z(&self, angle: f32) -> Self {
        self.from_matrix(&self.get_matrix().rotate_z(angle))
    }

    fn rotate(&self, angle: f32, axis: glm::Vec4) -> Self {
        self.from_matrix(&self.get_matrix().rotate(angle, axis))
    }

    fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        self.from_matrix(&self.get_matrix().scale(x, y, z))
    }
}
