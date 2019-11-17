extern crate gl;
extern crate glm;
extern crate glutin;
mod geo_objs;
mod matrix;
mod utils;
use geo_objs::Cube;
use glm::cos;
use glm::sin;
use glutin::dpi::LogicalSize;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use utils::compile_shader;
use utils::link_program;

fn main() {
    // Razão de proporção da janela (largura/altura). Veja função FramebufferSizeCallback().
    let g_screen_ratio = 1.0;

    // Variáveis que definem a câmera em coordenadas esféricas, controladas pelo
    // usuário através do mouse (veja função CursorPosCallback()). A posição
    // efetiva da câmera é calculada dentro da função main(), dentro do loop de
    // renderização.
    let g_camera_theta = 0.0; // Ângulo no plano ZX em relação ao eixo Z
    let g_camera_phi = 0.0; // Ângulo em relação ao eixo Y
    let g_camera_distance = 2.5; // Distância da câmera para a origem

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

    unsafe {
        gl::UseProgram(program);
        let mut cube = Cube::new();
        let mut model_uniform = 0i32;
        let mut view_uniform = 0i32;
        let mut projection_uniform = 0i32;
        // Habilita Zbuffer
        gl::Enable(gl::DEPTH_TEST);
        model_uniform = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
        view_uniform = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());
        projection_uniform =
            gl::GetUniformLocation(program, CString::new("projection").unwrap().as_ptr());

        events_loop.run_forever(|event| {
            use glutin::{ControlFlow, Event, WindowEvent};
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
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

            gl::BindVertexArray(cube.vao);

            // Computamos a posição da câmera utilizando coordenadas esféricas.  As
            // variáveis g_CameraDistance, g_CameraPhi, e g_CameraTheta são
            // controladas pelo mouse do usuário. Veja as funções CursorPosCallback()
            // e ScrollCallback().
            let r = g_camera_distance;
            let y = r * sin(g_camera_phi);
            let z = r * cos(g_camera_phi) * cos(g_camera_theta);
            let x = r * cos(g_camera_phi) * sin(g_camera_theta);

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
                matrix::perpective_matrix(field_of_view, g_screen_ratio, nearplane, farplane)
                    .matrix;

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

            let model = matrix::identity_matrix().matrix;
            gl::UniformMatrix4fv(model_uniform, 1, gl::FALSE, mem::transmute(&model[0]));

            cube.draw(&model_uniform);
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
