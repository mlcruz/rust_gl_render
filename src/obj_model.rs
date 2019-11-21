use complex_obj::ComplexObj;
use draw::Attach;
use draw::Draw;
use draw::DrawSelf;
use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use matrix::identity_matrix;
use matrix::GLMatrix;
use matrix::MatrixTransform;
use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::path::Path;
use std::ptr::null;
use tobj;

#[allow(dead_code)]
#[derive(Debug, Copy)]
pub struct ObjModel {
    pub model: GLMatrix,
    pub vao: u32,
    geometry_vbo: u32,
    color_vbo: u32,
    ebo: u32,
    index_len: usize,
}

#[allow(dead_code)]
impl ObjModel {
    pub fn new(path: &str) -> Self {
        // Carrega arquivo obj
        let (models, _materials) = tobj::load_obj(Path::new(path)).unwrap();

        let mut myself = ObjModel {
            vao: 0u32,
            geometry_vbo: 0u32,
            color_vbo: 0u32,
            ebo: 0u32,
            model: identity_matrix(),
            index_len: 0,
        };

        let mut position_array = Vec::new();
        let mut normal_array = Vec::new();
        let mut index_array = Vec::new();

        // Carrega dados de posições e indices para em vetores contínuos
        // 3 valores no vetor de indices representam os vertices de um indice
        // 4 valores no vetor de posição representam a posição de um vertice
        for (_index, model) in models.iter().enumerate() {
            let mesh = &model.mesh;

            for f in 0..mesh.indices.len() / 3 {
                // Vertices X Y Z de um triangulo
                index_array.push(mesh.indices[3 * f]);
                index_array.push(mesh.indices[3 * f + 1]);
                index_array.push(mesh.indices[3 * f + 2]);
            }
            for v in 0..mesh.positions.len() / 3 {
                // Insere uma posição de um vertice
                // X Y Z W em ordem
                position_array.push(mesh.positions[3 * v]);
                position_array.push(mesh.positions[3 * v + 1]);
                position_array.push(mesh.positions[3 * v + 2]);
                position_array.push(1f32);
            }

            // Verifica se existem normais no obj, e insere
            if mesh.normals.len() > 0 {
                for v in 0..mesh.normals.len() / 3 {
                    // Insere um normal de um vertice
                    // X Y Z W em ordem
                    normal_array.push(mesh.positions[3 * v]);
                    normal_array.push(mesh.positions[3 * v + 1]);
                    normal_array.push(mesh.positions[3 * v + 2]);
                    normal_array.push(0f32);
                }
            } else {
                for _v in 0..mesh.positions.len() / 3 {
                    // Se não existem normais, inicializa vetor vazio
                    normal_array.push(0f32);
                    normal_array.push(0f32);
                    normal_array.push(0f32);
                    normal_array.push(0f32);
                }
            }
        }

        // Alocação de VAO e VBOS
        unsafe {
            // Definição dos atributos dos vertices
            // Cria VAO do obj e "liga" ele
            gl::GenVertexArrays(1, &mut myself.vao);
            gl::BindVertexArray(myself.vao);

            // Cria identificador do VBO a ser utilizado pelos atributos de geometria e "liga" o mesmo
            gl::GenBuffers(1, &mut myself.geometry_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, myself.geometry_vbo);

            // Aloca memória para o VBO acima.
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (position_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, // Tamanho dos vertices
                null(),
                gl::STATIC_DRAW,
            );

            //Copia valores dos array de vertices para o VBO
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (position_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                position_array.as_ptr() as *const c_void,
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
            gl::GenBuffers(1, &mut myself.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, myself.ebo);

            // Aloca memória para o VBO  acima.
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (index_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, // Tamanho dos vertices
                null(),
                gl::STATIC_DRAW,
            );
            // Copia valores dos array de vertices para o VBO
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                0,
                (index_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                index_array.as_ptr() as *const c_void,
            );
            gl::BindVertexArray(0);
            myself.index_len = index_array.len();
        }
        myself
    }
}

impl MatrixTransform for ObjModel {
    fn get_matrix(&self) -> &GLMatrix {
        &self.model
    }
    fn update_matrix(&mut self, matrix: &GLMatrix) -> &Self {
        self.model = matrix.clone();
        self
    }
    fn from_matrix(&self, matrix: &GLMatrix) -> Self {
        *self.clone().update_matrix(matrix)
    }
}

impl Clone for ObjModel {
    fn clone(&self) -> Self {
        *self
    }
}

impl DrawSelf for ObjModel {
    fn draw_self(&self, program: &u32) -> &Self {
        unsafe {
            gl::UseProgram(*program);

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
                self.index_len as i32,
                gl::UNSIGNED_INT,
                0 as *const i32 as *const c_void,
            );
        }
        self
    }
}

impl Draw for ObjModel {
    fn draw(&self, program: &u32) {
        unsafe {
            gl::UseProgram(*program);

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
                self.index_len as i32,
                gl::UNSIGNED_INT,
                0 as *const i32 as *const c_void,
            );
        }
    }

    fn draw_with_transform(&self, matrix: GLMatrix, program: &u32) {
        let new_matrix = matrix.matrix * self.model.matrix;
        let mut new_me = self.clone();
        new_me.model.matrix = new_matrix;
        new_me.draw(program);
    }
}

impl<'a> Attach<'a> for ObjModel {
    fn attach(&'a self, child: &'a dyn Draw) -> ComplexObj {
        ComplexObj::new(self, vec![child.clone()], self.model)
    }
}
