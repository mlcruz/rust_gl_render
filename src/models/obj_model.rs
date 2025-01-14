use super::draw::Draw;
use super::matrix::compute_normal;
use super::matrix::identity_matrix;
use super::matrix::norm;
use super::matrix::GLMatrix;
use super::matrix::MatrixTransform;
use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use models::load_texture::load_texture;
use models::scene_object::check_bbox_bbox_intersection;
use models::scene_object::SceneObject;
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
    ebo: u32,
    geometry_vbo: u32,
    texture_vbo: u32,
    normal_vbo: u32,
    index_len: usize,
    pub bbox_min: glm::Vec3,
    pub bbox_max: glm::Vec3,
    pub texture_override: u32,
    pub phong_q_overide: f32,
    pub specular_reflectance_override: glm::Vec3,
    pub ambient_reflectance_overide: glm::Vec3,
    pub color_overide: glm::Vec3,
    pub texture_map_type: i32,
    pub lighting_source_override: glm::Vec4,
}

static ID_MATRIX: GLMatrix = identity_matrix();

#[allow(dead_code)]
impl ObjModel {
    pub fn new(path: &str) -> Self {
        // Carrega arquivo obj
        let (models, _materials) = tobj::load_obj(Path::new(path)).unwrap();

        // Objeto simples:
        // Define propriedades do arquivo obj que representa um obj na tela
        // vao -> Endereço da vao do objeto, não é criada em clones do obj
        // ebo -> Indices do obj
        // texture,normal,geometry vbo -> Auto descritivo
        // model: Matrix model inicial do obj. Padrão é matriz identidade
        // Tamanho do indice dos vertices do obj
        // bbox_min/max -> Bounding box computada na inicialização do obj
        // Texture overide -> Local da textura que sobreescreve a textura atual do obj se texture map type for setado.
        // Textura map type: Tipo de mapeamento da textura. 0 - Arquivo OBJ; 1- Planar XY; 2- Esferico; 3- Cilindrico
        let mut myself = ObjModel {
            vao: 0u32,
            ebo: 0u32,
            geometry_vbo: 0u32,
            texture_vbo: 0u32,
            normal_vbo: 0u32,
            model: ID_MATRIX,
            index_len: 0,
            bbox_min: glm::vec3(0.0, 0.0, 0.0),
            bbox_max: glm::vec3(0.0, 0.0, 0.0),
            texture_override: 0,
            texture_map_type: 0,
            specular_reflectance_override: glm::vec3(0.0, 0.0, 0.0),
            ambient_reflectance_overide: glm::vec3(0.0, 0.0, 0.0),
            phong_q_overide: 1.0,
            color_overide: glm::vec3(0.0, 0.0, 0.0),
            lighting_source_override: glm::vec4(0.0, 0.0, 0.0, 0.0),
        };

        let mut position_array = Vec::new();
        let mut normal_array = Vec::new();
        let mut index_array = Vec::new();
        let mut texture_array = Vec::new();

        // Carrega dados de posições e indices para em vetores contínuos
        // 3 valores no vetor de indices representam os vertices de um indice
        // 4 valores no vetor de posição representam a posição de um vertice
        for (_index, model) in models.iter().enumerate() {
            let mesh = &model.mesh;

            for f in 0..mesh.indices.len() {
                // Vertices X Y Z de um triangulo
                index_array.push(mesh.indices[f]);
            }

            for v in 0..mesh.positions.len() / 3 {
                // Insere uma posição de um vertice
                // X Y Z W em ordem
                let x = mesh.positions[3 * v];
                let y = mesh.positions[3 * v + 1];
                let z = mesh.positions[3 * v + 2];

                position_array.push(x);
                position_array.push(y);
                position_array.push(z);
                position_array.push(1f32);

                myself.bbox_min.x = glm::min(myself.bbox_min.x, x);
                myself.bbox_min.y = glm::min(myself.bbox_min.y, y);
                myself.bbox_min.z = glm::min(myself.bbox_min.z, z);
                myself.bbox_max.x = glm::max(myself.bbox_max.x, x);
                myself.bbox_max.y = glm::max(myself.bbox_max.y, y);
                myself.bbox_max.z = glm::max(myself.bbox_max.z, z);
            }

            // Insere texturas
            if mesh.texcoords.len() > 0 {
                for v in 0..mesh.texcoords.len() {
                    texture_array.push(mesh.texcoords[v]);
                }
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
                // Computa normais dos vertices se não existe no obj
                let num_vertices = mesh.positions.len() / 3;

                let mut num_triangles_per_vertex: Vec<u32> = vec![0; num_vertices];

                let mut vertex_normals: Vec<glm::Vec4> =
                    vec![glm::Vec4::new(0f32, 0f32, 0f32, 1f32); num_vertices];

                //println!("{:?}", mesh.indices.len() / 3);
                for t in 0..mesh.indices.len() / 3 {
                    // Calcula a normal de todos os triangulos

                    let mut triangle_vertex_array = [
                        glm::Vec4::new(0f32, 0f32, 0f32, 1f32),
                        glm::Vec4::new(0f32, 0f32, 0f32, 1f32),
                        glm::Vec4::new(0f32, 0f32, 0f32, 1f32),
                    ];

                    for v in 0..3 {
                        let idx = mesh.indices[3 * t + v] as usize;
                        let vx = mesh.positions[3 * idx + 0];
                        let vy = mesh.positions[3 * idx + 1];
                        let vz = mesh.positions[3 * idx + 2];

                        triangle_vertex_array[v].x = vx;
                        triangle_vertex_array[v].y = vy;
                        triangle_vertex_array[v].z = vz;
                    }

                    let [a, b, c] = triangle_vertex_array;
                    let n = compute_normal(&a, &b, &c);
                    for v in 0..3 {
                        let idx = mesh.indices[3 * t + v] as usize;
                        num_triangles_per_vertex[idx] = num_triangles_per_vertex[idx] + 1;
                        vertex_normals[idx] = vertex_normals[idx] + n;
                    }
                }

                for i in 0..vertex_normals.len() {
                    let mut n = vertex_normals[i] / num_triangles_per_vertex[i] as f32;
                    n = n / norm(n);

                    normal_array.push(n.x);
                    normal_array.push(n.y);
                    normal_array.push(n.z);
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

            //Normais:
            if normal_array.len() > 0 {
                gl::GenBuffers(1, &mut myself.normal_vbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, myself.normal_vbo);

                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (normal_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    null(),
                    gl::STATIC_DRAW,
                );
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (normal_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    normal_array.as_ptr() as *const c_void,
                );

                // Location das normais no shader
                let location: GLuint = 1;

                gl::VertexAttribPointer(location, 4, gl::FLOAT, gl::FALSE, 0, null());
                gl::EnableVertexAttribArray(location);
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            }

            // Aloca texturas
            if texture_array.len() > 0 {
                gl::GenBuffers(1, &mut myself.texture_vbo);
                gl::BindBuffer(gl::ARRAY_BUFFER, myself.texture_vbo);

                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (texture_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    null(),
                    gl::STATIC_DRAW,
                );
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (texture_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    texture_array.as_ptr() as *const c_void,
                );

                // Location das texturas no shader
                let location: GLuint = 2;

                gl::VertexAttribPointer(location, 4, gl::FLOAT, gl::FALSE, 0, null());
                gl::EnableVertexAttribArray(location);
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            }

            // Topolgia:
            gl::GenBuffers(1, &mut myself.ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, myself.ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (index_array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                null(),
                gl::STATIC_DRAW,
            );
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

    pub fn with_texture(&self, texture: &u32, texture_map_type: i32) -> Self {
        Self {
            texture_override: *texture,
            texture_map_type: texture_map_type,
            ..*self
        }
    }

    pub fn with_texture_map_type(&self, texture_map_type: i32) -> Self {
        Self {
            texture_map_type: texture_map_type,
            ..*self
        }
    }

    pub unsafe fn load_texture(&self, path: &str) -> Self {
        let (tex, _) = load_texture(path);
        Self {
            texture_override: tex,
            ..*self
        }
    }

    pub fn with_specular_reflectance(&self, specular_reflectance: &glm::Vec3) -> Self {
        Self {
            specular_reflectance_override: *specular_reflectance,
            ..*self
        }
    }

    pub fn with_lighting_source_override(&self, lighting_source_override: &glm::Vec4) -> Self {
        Self {
            lighting_source_override: *lighting_source_override,
            ..*self
        }
    }

    pub fn with_ambient_reflectance(&self, ambient_reflectance: &glm::Vec3) -> Self {
        Self {
            ambient_reflectance_overide: *ambient_reflectance,
            ..*self
        }
    }
    pub fn with_specular_phong_q(&self, phong_q: &f32) -> Self {
        Self {
            phong_q_overide: *phong_q,
            ..*self
        }
    }

    pub fn with_color(&self, color: &glm::Vec3) -> Self {
        Self {
            color_overide: *color,
            ..*self
        }
    }

    pub fn check_intersection(&self, obj2: &SceneObject) -> bool {
        let obj1 = self;

        // Utiliza coluna de translação da matrix do obj para calcular pos global
        let obj1_t = obj1.get_matrix().matrix.c3;
        let obj2_t = obj2.get_matrix().matrix.c3;

        let obj1_bbox_min = obj1.bbox_min;
        let obj1_bbox_max = obj1.bbox_max;
        let obj2_bbox_min = obj2.get_bbox_min();
        let obj2_bbox_max = obj2.get_bbox_max();

        // Pos global da bbox  do obj1
        let obj1_bbox_min_pos = obj1.get_matrix().matrix
            * glm::vec4(obj1_bbox_min.x, obj1_bbox_min.y, obj1_bbox_min.z, 0.0)
            + obj1_t;
        let obj1_bbox_max_pos = obj1.get_matrix().matrix
            * glm::vec4(obj1_bbox_max.x, obj1_bbox_max.y, obj1_bbox_max.z, 0.0)
            + obj1_t;

        // Pos global da bbox do obj2
        let obj2_bbox_min_pos = obj1.get_matrix().matrix
            * glm::vec4(obj2_bbox_min.x, obj2_bbox_min.y, obj2_bbox_min.z, 0.0)
            + obj2_t;
        let obj2_bbox_max_pos = obj1.get_matrix().matrix
            * glm::vec4(obj2_bbox_max.x, obj2_bbox_max.y, obj2_bbox_max.z, 0.0)
            + obj2_t;

        check_bbox_bbox_intersection(
            &obj1_bbox_min_pos,
            &obj1_bbox_max_pos,
            &obj2_bbox_min_pos,
            &obj2_bbox_max_pos,
        )
    }
}

impl MatrixTransform for ObjModel {
    fn get_matrix(&self) -> &GLMatrix {
        &self.model
    }
    fn update_matrix(&mut self, matrix: &GLMatrix) -> &mut Self {
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

// Desenha objeto na tela
impl Draw for ObjModel {
    fn draw(&self, program: &u32) -> &Self {
        unsafe {
            gl::UseProgram(*program);

            gl::BindVertexArray(self.vao);

            // Carrega uniforms com atributos do objeto definidos na inicialização
            gl::Uniform1i(
                gl::GetUniformLocation(*program, CString::new("texture_overide").unwrap().as_ptr()),
                self.texture_override as i32,
            );
            let texture_map_type_uniform = gl::GetUniformLocation(
                *program,
                CString::new("texture_map_type").unwrap().as_ptr(),
            );

            let color_overide_uniform =
                gl::GetUniformLocation(*program, CString::new("color_overide").unwrap().as_ptr());

            let model_uniform =
                gl::GetUniformLocation(*program, CString::new("model").unwrap().as_ptr());

            let bbox_min_uniform =
                gl::GetUniformLocation(*program, CString::new("bbox_min").unwrap().as_ptr());

            let bbox_max_uniform =
                gl::GetUniformLocation(*program, CString::new("bbox_max").unwrap().as_ptr());

            let specular_reflectance_uniform = gl::GetUniformLocation(
                *program,
                CString::new("specular_reflectance").unwrap().as_ptr(),
            );
            let lighting_direction_uniform = gl::GetUniformLocation(
                *program,
                CString::new("lighting_source_override").unwrap().as_ptr(),
            );

            let ambient_reflectance_uniform = gl::GetUniformLocation(
                *program,
                CString::new("ambient_reflectance").unwrap().as_ptr(),
            );

            let phong_q_uniform =
                gl::GetUniformLocation(*program, CString::new("phong_q").unwrap().as_ptr());

            // Setamos as variáveis "bbox_min" e "bbox_max" do fragment shader
            // com os parâmetros da axis-aligned bounding box (AABB) do modelo.
            gl::Uniform4f(
                bbox_min_uniform,
                self.bbox_min.x,
                self.bbox_min.y,
                self.bbox_min.z,
                1.0,
            );

            gl::Uniform1i(texture_map_type_uniform, self.texture_map_type);

            gl::Uniform4f(
                bbox_max_uniform,
                self.bbox_max.x,
                self.bbox_max.y,
                self.bbox_max.z,
                1.0,
            );
            gl::Uniform4f(
                lighting_direction_uniform,
                self.lighting_source_override.x,
                self.lighting_source_override.y,
                self.lighting_source_override.z,
                1.0,
            );

            gl::Uniform3f(
                color_overide_uniform,
                self.color_overide.x,
                self.color_overide.y,
                self.color_overide.z,
            );

            gl::Uniform3f(
                specular_reflectance_uniform,
                self.specular_reflectance_override.x,
                self.specular_reflectance_override.y,
                self.specular_reflectance_override.z,
            );

            gl::Uniform3f(
                ambient_reflectance_uniform,
                self.ambient_reflectance_overide.x,
                self.ambient_reflectance_overide.y,
                self.ambient_reflectance_overide.z,
            );

            gl::UniformMatrix4fv(
                model_uniform,
                1,
                gl::FALSE,
                mem::transmute(&self.model.matrix[0]),
            );
            gl::Uniform1f(phong_q_uniform, self.phong_q_overide);

            // Desenha elemento
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
