extern crate gl;
extern crate glm;
extern crate glutin;
mod geo_objs;
mod matrix;
mod utils;
use geo_objs::Cube;
use gl::types::GLfloat;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use glm::cos;
use glm::sin;
use glutin::dpi::LogicalSize;
use matrix::camera_view_matrix;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::ptr::null;
use utils::compile_shader;
use utils::link_program;

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

fn main() {
    // Razão de proporção da janela (largura/altura). Veja função FramebufferSizeCallback().
    let g_ScreenRatio = 1.0;

    // Ângulos de Euler que controlam a rotação de um dos cubos da cena virtual
    let g_AngleX = 0.0;
    let g_AngleY = 0.0;
    let g_AngleZ = 0.0;

    // "g_LeftMouseButtonPressed = true" se o usuário está com o botão esquerdo do mouse
    // pressionado no momento atual. Veja função MouseButtonCallback().
    let g_LeftMouseButtonPressed = false;

    // Variáveis que definem a câmera em coordenadas esféricas, controladas pelo
    // usuário através do mouse (veja função CursorPosCallback()). A posição
    // efetiva da câmera é calculada dentro da função main(), dentro do loop de
    // renderização.
    let g_CameraTheta = 0.0; // Ângulo no plano ZX em relação ao eixo Z
    let g_CameraPhi = 0.0; // Ângulo em relação ao eixo Y
    let g_CameraDistance = 2.5; // Distância da câmera para a origem

    // Variável que controla o tipo de projeção utilizada: perspectiva ou ortográfica.
    let g_UsePerspectiveProjection = true;

    // Variável que controla se o texto informativo será mostrado na tela.
    let g_ShowInfoText = true;

    // Carrega shaders do arquivo
    let vertex_path = Path::new("src/shader/vertex.glsl");
    let fragment_path = Path::new("src/shader/fragment.glsl");
    let mut vertex_shader = String::new();
    let mut fragment_shader = String::new();

    let mut fragment_file = match File::open(&fragment_path) {
        Err(_err) => panic!("Cade o shader"),
        Ok(file) => file,
    };

    let mut vertex_file = match File::open(&vertex_path) {
        Err(_err) => panic!("Cade o shader"),
        Ok(file) => file,
    };

    fragment_file.read_to_string(&mut fragment_shader).unwrap();
    vertex_file.read_to_string(&mut vertex_shader).unwrap();

    // Inicializa loop de eventos da janela
    let mut events_loop = glutin::EventsLoop::new();

    // Iniciliza janela, com perfil core, versão 3.3, tamanho 800x600
    let window = glutin::WindowBuilder::new()
        .with_title("Trabalho final fcg")
        .with_dimensions(<LogicalSize>::new(800.0f64, 600.0f64));

    let gl_window = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(window, &events_loop)
        .unwrap();

    // Coloca janela no contexto atual
    let gl_window = unsafe { gl_window.make_current() }.unwrap();

    // Carrega ponteiros para funções do openGL
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    // Compila shaders e linka shaders
    let vs = compile_shader(&vertex_shader, gl::VERTEX_SHADER);
    let fs = compile_shader(&fragment_shader, gl::FRAGMENT_SHADER);
    let program = link_program(vs, fs);

    let mut model_uniform = 0i32;
    let mut view_uniform = 0i32;
    let mut projection_uniform = 0i32;

    unsafe {
        // Habilita Zbuffer
        gl::Enable(gl::DEPTH_TEST);

        model_uniform = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
        view_uniform = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());

        projection_uniform =
            gl::GetUniformLocation(program, CString::new("projection").unwrap().as_ptr());

        static mut vao: GLuint = 0;
        static mut geometry_vbo: GLuint = 0;
        static mut color_vbo: GLuint = 0;
        static mut topology_vbo: GLuint = 0;

        // Definição dos atributos dos vertices
        // Cria VAO do cubo e "liga" ele
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Cria identificador do VBO a ser utilizado pelos atributos de geometria e "liga" o mesmo
        gl::GenBuffers(1, &mut geometry_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, geometry_vbo);

        let geometry_size = (CUBE_VERTEX_GEOMETRY.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
        let topology_size = (CUBE_VERTEX_TOPOLOGY.len() * mem::size_of::<GLuint>()) as GLsizeiptr;

        // Aloca memória para o VBO acima.
        gl::BufferData(
            gl::ARRAY_BUFFER,
            geometry_size, // Tamanho dos vertices
            null(),
            gl::STATIC_DRAW,
        );
        // Copia valores dos array de vertices para o VBO
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            geometry_size,
            mem::transmute(&CUBE_VERTEX_GEOMETRY[0]),
        );

        // Location no shader para o VBO acima
        let location: GLuint = 0; // location 0 no vertex shader
                                  //let ptr_offset: *const std::ffi::c_void = 0 as *const std::ffi::c_void;

        // "Liga" VAO e VBO
        gl::VertexAttribPointer(location, 4, gl::FLOAT, gl::FALSE, 0, null());
        // Ativa atributos
        gl::EnableVertexAttribArray(location);
        // Desliga VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // Topolgia:
        // Cria identificador do VBO a ser utilizado pela topologia e "liga" o mesmo
        gl::GenBuffers(1, &mut topology_vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, topology_vbo);

        // Aloca memória para o VBO  acima.
        gl::BufferData(
            gl::ARRAY_BUFFER,
            topology_size, // Tamanho dos vertices
            null(),
            gl::STATIC_DRAW,
        );
        // Copia valores dos array de vertices para o VBO
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            topology_size,
            mem::transmute(&CUBE_VERTEX_TOPOLOGY[0]),
        );
        // Desliga VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        events_loop.run_forever(|event| {
            use glutin::{ControlFlow, Event, WindowEvent};
            let mut control_flow_state = ControlFlow::Continue;
            // Handling de eventos
            match event {
                Event::WindowEvent { event, .. } => match event {
                    // Em caso de event de fechamento de tela, seta controle do loop de eventos para encerrar
                    WindowEvent::CloseRequested => control_flow_state = ControlFlow::Break,
                    _ => (),
                },
                _ => (),
            }

            // Renderização
            // gl::BindVertexArray(*cube.vao);
            gl::BindVertexArray(vao);

            gl::DrawElements(
                gl::TRIANGLES, // Veja slide 150 do documento "Aula_04_Modelagem_Geometrica_3D.pdf"
                10,
                gl::UNSIGNED_INT,
                mem::transmute(&CUBE_VERTEX_TOPOLOGY[0]),
            );
            // Computamos a posição da câmera utilizando coordenadas esféricas.  As
            // variáveis g_CameraDistance, g_CameraPhi, e g_CameraTheta são
            // controladas pelo mouse do usuário. Veja as funções CursorPosCallback()
            // e ScrollCallback().
            let r = g_CameraDistance;
            let y = r * sin(g_CameraPhi);
            let z = r * cos(g_CameraPhi) * cos(g_CameraTheta);
            let x = r * cos(g_CameraPhi) * sin(g_CameraTheta);

            // Abaixo definimos as varáveis que efetivamente definem a câmera virtual.
            // Veja slides 172-182 do documento "Aula_08_Sistemas_de_Coordenadas.pdf".
            let camera_position_c = glm::vec4(x, y, z, 1.0); // Ponto "c", centro da câmera
            let camera_lookat_l = glm::vec4(0.0, 0.0, 0.0, 1.0); // Ponto "l", para onde a câmera (look-at) estará sempre olhando
            let camera_view_vector = camera_lookat_l - camera_position_c; // Vetor "view", sentido para onde a câmera está virada
            let camera_up_vector = glm::vec4(0.0, 1.0, 0.0, 0.0); // Vetor "up" fixado para apontar para o "céu" (eito Y global)

            // Computamos a matriz "View" utilizando os parâmetros da câmera para
            // definir o sistema de coordenadas da câmera.  Veja slide 186 do documento "Aula_08_Sistemas_de_Coordenadas.pdf".
            let view =
                matrix::camera_view_matrix(camera_position_c, camera_view_vector, camera_up_vector)
                    .matrix;

            // Note que, no sistema de coordenadas da câmera, os planos near e far
            // estão no sentido negativo! Veja slides 190-193 do documento "Aula_09_Projecoes.pdf".
            let nearplane = -0.1; // Posição do "near plane"
            let farplane = -10.0; // Posição do "far plane"

            let field_of_view = 3.141592 / 3.0;
            let projection =
                matrix::perpective_matrix(field_of_view, g_ScreenRatio, nearplane, farplane).matrix;

            // Enviamos as matrizes "view" e "projection" para a placa de vídeo
            // (GPU). Veja o arquivo "shader_vertex.glsl", onde estas são
            // efetivamente aplicadas em todos os pontos.
            gl::UniformMatrix4fv(view_uniform, 1, gl::FALSE, mem::transmute(&view[0]));
            gl::UniformMatrix4fv(
                projection_uniform,
                1,
                gl::FALSE,
                mem::transmute(&projection[0]),
            );

            let model = matrix::identity_matrix();

            // Enviamos a matriz "model" para a placa de vídeo (GPU). Veja o
            // arquivo "shader_vertex.glsl", onde esta é efetivamente
            // aplicada em todos os pontos.
            gl::UniformMatrix4fv(
                model_uniform,
                1,
                gl::FALSE,
                mem::transmute(&model.matrix[0]),
            );
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::UseProgram(program);
            // gl::DrawElements(
            //     gl::TRIANGLES, // Veja slide 150 do documento "Aula_04_Modelagem_Geometrica_3D.pdf"
            //     10,
            //     gl::UNSIGNED_INT,
            //     null(),
            // );
            gl_window.swap_buffers().unwrap();

            control_flow_state
        });
    }

    // Cleanup
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
    }
}
