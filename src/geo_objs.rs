use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use matrix::identity_matrix;
use matrix::GLMatrix;
use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::ptr::null;

static CUBE_VERTEX_GEOMETRY: [GLfloat; 56] = [
    -0.5, 0.5, 0.5, 1.0, // posição do vértice 0
    -0.5, -0.5, 0.5, 1.0, // posição do vértice 1
    0.5, -0.5, 0.5, 1.0, // posição do vértice 2
    0.5, 0.5, 0.5, 1.0, // posição do vértice 3
    -0.5, 0.5, -0.5, 1.0, // posição do vértice 4
    -0.5, -0.5, -0.5, 1.0, // posição do vértice 5
    0.5, -0.5, -0.5, 1.0, // posição do vértice 6
    0.5, 0.5, -0.5, 1.0, // posição do vértice 7
    // Vértices para desenhar o eixo X
    //    X      Y     Z     W
    0.0, 0.0, 0.0, 1.0, // posição do vértice 8
    1.0, 0.0, 0.0, 1.0, // posição do vértice 9
    // Vértices para desenhar o eixo Y
    //    X      Y     Z     W
    0.0, 0.0, 0.0, 1.0, // posição do vértice 10
    0.0, 1.0, 0.0, 1.0, // posição do vértice 11
    // Vértices para desenhar o eixo Z
    //    X      Y     Z     W
    0.0, 0.0, 0.0, 1.0, // posição do vértice 12
    0.0, 0.0, 1.0, 1.0, // posição do vértice 13
];

static CUBE_VERTEX_TOPOLOGY: [i32; 66] = [
    0, 1, 2, // triângulo 1
    7, 6, 5, // triângulo 2
    3, 2, 6, // triângulo 3
    4, 0, 3, // triângulo 4
    4, 5, 1, // triângulo 5
    1, 5, 6, // triângulo 6
    0, 2, 3, // triângulo 7
    7, 5, 4, // triângulo 8
    3, 6, 7, // triângulo 9
    4, 3, 7, // triângulo 10
    4, 1, 0, // triângulo 11
    1, 6, 2, // triângulo 12
    // Definimos os índices dos vértices que definem as ARESTAS de um cubo
    // através de 12 linhas que serão desenhadas com o modo de renderização
    // GL_LINES.
    0, 1, // linha 1
    1, 2, // linha 2
    2, 3, // linha 3
    3, 0, // linha 4
    0, 4, // linha 5
    4, 7, // linha 6
    7, 6, // linha 7
    6, 2, // linha 8
    6, 5, // linha 9
    5, 4, // linha 10
    5, 1, // linha 11
    7, 3, // linha 12
    // Definimos os índices dos vértices que definem as linhas dos eixos X, Y,
    // Z, que serão desenhados com o modo GL_LINES.
    8, 9, // linha 1
    10, 11, // linha 2
    12, 13, // linha 3,
];

static GEOMETRY_SIZE: GLsizeiptr =
    (CUBE_VERTEX_GEOMETRY.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
static TOPOLOGY_SIZE: GLsizeiptr =
    (CUBE_VERTEX_TOPOLOGY.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;

#[derive(Copy, Debug)]
#[allow(dead_code)]
pub struct Cube {
    pub vao: u32,
    ebo: u32,
    color_vbo: u32,
    topology_vbo: u32,
    pub model: GLMatrix,
}

#[allow(dead_code)]
impl Cube {
    pub fn new() -> Self {
        let mut myself = Cube {
            vao: 0u32,
            ebo: 0u32,
            color_vbo: 0u32,
            topology_vbo: 0u32,
            model: identity_matrix(),
        };

        unsafe {
            // Definição dos atributos dos vertices
            // Cria VAO do cubo e "liga" ele
            gl::GenVertexArrays(1, &mut myself.vao);
            gl::BindVertexArray(myself.vao);

            // Cria identificador do VBO a ser utilizado pelos atributos de geometria e "liga" o mesmo
            gl::GenBuffers(1, &mut myself.ebo);
            gl::BindBuffer(gl::ARRAY_BUFFER, myself.ebo);

            // Aloca memória para o VBO acima.
            gl::BufferData(
                gl::ARRAY_BUFFER,
                GEOMETRY_SIZE, // Tamanho dos vertices
                null(),
                gl::STATIC_DRAW,
            );
            // Copia valores dos array de vertices para o VBO
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                GEOMETRY_SIZE,
                &CUBE_VERTEX_GEOMETRY[0] as *const f32 as *const c_void,
            );

            // Location no shader para o VBO acima
            let location: GLuint = 0; // location 0 no vertex shader

            // "Liga" VAO e VBO
            gl::VertexAttribPointer(location, 4, gl::FLOAT, gl::FALSE, 0, null());

            // Ativa atributos
            gl::EnableVertexAttribArray(location);
            // Desliga VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // Topolgia:
            // Cria identificador do VBO a ser utilizado pela topologia e "liga" o mesmo
            gl::GenBuffers(1, &mut myself.topology_vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, myself.topology_vbo);

            // Aloca memória para o VBO  acima.
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                TOPOLOGY_SIZE, // Tamanho dos vertices
                null(),
                gl::STATIC_DRAW,
            );
            // Copia valores dos array de vertices para o VBO
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                0,
                TOPOLOGY_SIZE,
                &CUBE_VERTEX_TOPOLOGY[0] as *const i32 as *const c_void,
            );

            gl::BindVertexArray(0);
        }
        myself
    }
    #[allow(unused_variables)]
    pub fn draw(&self, program: &u32) -> Self {
        let cube_face_first_index = 0;
        let cube_face_length = 36;

        let cube_edges_first_index = 36;
        let cube_edges_length = 24;

        let cube_axis_first_index = 60;
        let cube_axis_length = 6;

        let c_ptr_offset: *const std::ffi::c_void =
            cube_axis_first_index as *const std::ffi::c_void;

        unsafe {
            // Enviamos a matriz "model" para a placa de vídeo (GPU). Veja o
            // arquivo "shader_vertex.glsl", onde esta é efetivamente
            // aplicada em todos os pontos.

            gl::BindVertexArray(self.vao);
            let model_uniform =
                gl::GetUniformLocation(*program, CString::new("model").unwrap().as_ptr());

            gl::UniformMatrix4fv(
                model_uniform,
                1,
                gl::FALSE,
                mem::transmute(&self.model.matrix[0]),
            );

            gl::DrawElements(
                gl::TRIANGLES,
                cube_face_length,
                gl::UNSIGNED_INT,
                0 as *const i32 as *const c_void,
            );
            // Pedimos para OpenGL desenhar linhas com largura de 4 pixels.
            gl::LineWidth(4.0);

            gl::DrawElements(
                gl::LINES,
                cube_axis_length,
                gl::UNSIGNED_INT,
                cube_axis_first_index as *const i32 as *const c_void,
            );

            gl::DrawElements(
                gl::LINES,
                cube_edges_length,
                gl::UNSIGNED_INT,
                cube_edges_first_index as *const i32 as *const c_void,
            );

            gl::BindVertexArray(0);
            *self
        }
    }
}

impl Clone for Cube {
    fn clone(&self) -> Self {
        Cube {
            vao: self.vao,
            color_vbo: self.color_vbo,
            ebo: self.ebo,
            topology_vbo: self.topology_vbo,
            model: self.model,
        }
    }
}
