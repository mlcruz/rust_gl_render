use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use matrix::cross_product;
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
#[derive(Debug, Clone)]
pub struct ObjModel {
    vertex_data: Vec<GLfloat>,
    normal_data: Vec<GLfloat>,
    index_data: Vec<u32>,
    pub model: GLMatrix,
    pub vao: u32,
    geometry_vbo: u32,
    color_vbo: u32,
    ebo: u32,
}

#[allow(dead_code)]
impl ObjModel {
    pub fn new(path: &str) -> Self {
        let (models, _materials) = tobj::load_obj(Path::new(path)).unwrap();

        let mut myself = ObjModel {
            vao: 0u32,
            geometry_vbo: 0u32,
            color_vbo: 0u32,
            ebo: 0u32,
            model: identity_matrix(),
            vertex_data: Vec::new(),
            normal_data: Vec::new(),
            index_data: Vec::new(),
        };

        let mut vertex_array = Vec::new();
        let mut normal_array = Vec::new();
        let mut index_array = Vec::new();
        // Carrega arquivo obj inicializa dados de vertices
        for model in &models {
            for index in &model.mesh.indices {
                index_array.push(*index);
                let i = *index as usize;
                let pos = [
                    &model.mesh.positions[3 * i],
                    &model.mesh.positions[3 * i + 1],
                    &model.mesh.positions[3 * i + 2],
                ];
                // let x = *(&model.mesh.positions[2 + *index as usize]);
                // let y = *(&model.mesh.positions[1 + *index as usize]);
                // let z = *(&model.mesh.positions[*index as usize]);
                let w = 1f32;

                // println!("{:?} {:?} {:?} {:?}", index, x, y, z);
                if !*(&model.mesh.normals.is_empty()) {
                    normal_array.push(*(&model.mesh.normals[3 * *index as usize]));
                    normal_array.push(*(&model.mesh.normals[1 + 3 * *index as usize]));
                    normal_array.push(*(&model.mesh.normals[2 + 3 * *index as usize]));
                } else {
                    normal_array.push(0f32);
                    normal_array.push(0f32);
                    normal_array.push(0f32);
                };

                vertex_array.push(*pos[0]);
                vertex_array.push(*pos[1]);
                vertex_array.push(*pos[2]);
                vertex_array.push(w);
            }
        }
        myself.vertex_data = vertex_array;
        myself.normal_data = normal_array;
        myself.index_data = index_array;
        // Alocação de buffers e etc
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
                (myself.vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, // Tamanho dos vertices
                null(),
                gl::STATIC_DRAW,
            );

            //Copia valores dos array de vertices para o VBO
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (myself.vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                myself.vertex_data.as_ptr() as *const c_void,
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
                (myself.index_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, // Tamanho dos vertices
                null(),
                gl::STATIC_DRAW,
            );
            // Copia valores dos array de vertices para o VBO
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                0,
                (myself.index_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                myself.index_data.as_ptr() as *const c_void,
            );
            gl::BindVertexArray(0);
        }
        myself
    }

    fn compute_normal(p1: &glm::Vec4, p2: &glm::Vec4, p3: &glm::Vec4) -> glm::Vec4 {
        let u = *p3 - *p1;
        let v = *p2 - *p1;
        cross_product(u, v)
    }

    pub fn draw(&self, program: &u32) {
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
                self.index_data.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const i32 as *const c_void,
            );
        }
    }
}

impl MatrixTransform for ObjModel {
    fn get_matrix(&self) -> GLMatrix {
        self.model
    }
    fn update_matrix(&mut self, matrix: &GLMatrix) {
        self.model = matrix.clone();
    }
    fn from_matrix(&self, matrix: &GLMatrix) -> Self {
        ObjModel {
            model: matrix.clone(),
            vertex_data: self.vertex_data.clone(),
            color_vbo: self.color_vbo,
            ebo: self.ebo,
            geometry_vbo: self.geometry_vbo,
            vao: self.vao,
            normal_data: self.normal_data.clone(),
            index_data: self.index_data.clone(),
        }
    }
}
