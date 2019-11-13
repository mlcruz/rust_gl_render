use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
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

static CUBE_VERTEX_TOPOLOGY: [GLuint; 66] = [
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

#[allow(dead_code)]
pub struct Cube {
    geometry: [GLfloat; 56],
    topology: [GLuint; 66],
    vao: u32,
    geometry_vbo: u32,
    color_vbo: u32,
    topology_vbo: u32,
    geometry_size: GLsizeiptr,
    topology_size: GLsizeiptr,
}

#[allow(dead_code)]
impl Cube {
    pub fn new() -> Self {
        let mut myself = Cube {
            vao: 0u32,
            geometry_vbo: 0u32,
            color_vbo: 0u32,
            topology_vbo: 0u32,
            geometry: CUBE_VERTEX_GEOMETRY,
            topology: CUBE_VERTEX_TOPOLOGY,
            geometry_size: (CUBE_VERTEX_GEOMETRY.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            topology_size: (CUBE_VERTEX_TOPOLOGY.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
        };

        unsafe {
            // Definição dos atributos dos vertices
            // Cria VAO do cubo e "liga" ele
            gl::GenVertexArrays(1, &mut myself.vao);
            gl::BindVertexArray(myself.vao);

            // Cria identificador do VBO a ser utilizado pelos atributos de geometria e "liga" o mesmo
            gl::GenBuffers(1, &mut myself.geometry_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, myself.geometry_vbo);

            // Aloca memória para o VBO acima.
            gl::BufferData(
                gl::ARRAY_BUFFER,
                myself.geometry_size, // Tamanho dos vertices
                null(),
                gl::STATIC_DRAW,
            );
            // Copia valores dos array de vertices para o VBO
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                myself.geometry_size,
                mem::transmute(&myself.geometry[0]),
            );

            // Location no shader para o VBO acima
            let location: GLuint = 0; // location 0 no vertex shader
            let ptr_offset: *const std::ffi::c_void = 0 as *const std::ffi::c_void;

            // "Liga" VAO e VBO
            gl::VertexAttribPointer(location, 4, gl::FLOAT, gl::FALSE, 0, ptr_offset);
            // Ativa atributos
            gl::EnableVertexAttribArray(location);
            // Desliga VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            // Topolgia:
            // Cria identificador do VBO a ser utilizado pela topologia e "liga" o mesmo
            gl::GenBuffers(1, &mut myself.topology_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, myself.topology_vbo);

            // Aloca memória para o VBO  acima.
            gl::BufferData(
                gl::ARRAY_BUFFER,
                myself.topology_size, // Tamanho dos vertices
                null(),
                gl::STATIC_DRAW,
            );
            // Copia valores dos array de vertices para o VBO
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                myself.topology_size,
                mem::transmute(&myself.geometry[0]),
            );
            // Desliga VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
        myself
    }
    #[allow(unused_variables)]
    pub fn draw(&mut self, program: u32) {
        unsafe {
            gl::UseProgram(program);
        }
        let cube_face_first_index: *const std::ffi::c_void = 0 as *const std::ffi::c_void;
        let cube_face_length = 36;

        let cube_edges_first_index: *const std::ffi::c_void =
            (36 * mem::size_of::<GLuint>()) as *const std::ffi::c_void;
        let cube_edges_length = 24;

        let cube_axis_first_index: *const std::ffi::c_void =
            (60 * mem::size_of::<GLuint>()) as *const std::ffi::c_void;
        let cube_axis_length = 6;
    }
}
