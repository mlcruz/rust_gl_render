extern crate gl;
extern crate glm;
extern crate glutin;
extern crate tobj;
mod camera;
mod complex_obj;
mod cube;
mod matrix;
mod obj_model;
mod shader_program;
mod utils;
mod vertex;
mod view;
use camera::Camera;
use cube::Cube;
use glutin::dpi::LogicalSize;
use matrix::MatrixTransform;
use obj_model::ObjModel;
use shader_program::Shader;
use view::View;

fn main() {
    // Variáveis que definem a câmera em coordenadas esféricas
    let g_camera_theta = 0.0; // Ângulo no plano ZX em relação ao eixo Z
    let g_camera_phi = 0.0; // Ângulo em relação ao eixo Y
    let g_camera_distance = 2.5; // Distância da câmera para a origem

    // Inicializa loop de eventos da janela
    let mut events_loop = glutin::EventsLoop::new();

    // Iniciliza janela e contexto, com perfil core, versão 3.3, tamanho 800x600
    let window = glutin::WindowBuilder::new()
        .with_title("Rust Render")
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

    // Compila e linka shaders
    let program = Shader::new("src/shader/vertex.glsl", "src/shader/fragment.glsl").program;

    // Inicializa camera
    let mut camera = Camera::new(g_camera_theta, g_camera_phi, g_camera_distance);

    // Inicializa matrizes de view e projeção com a camera criada
    let mut view = View::new(-0.01, -10.0, &camera);
    let mut is_view_orto = false;
    unsafe {
        gl::UseProgram(program);

        // Habilita Zbuffer
        gl::Enable(gl::DEPTH_TEST);

        // Inicializa uma vaca
        let cow = ObjModel::new("src/cow.obj");

        // Inicializa um cubo
        let cube = Cube::new();
        let mut should_break = false;

        loop {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);

            // Trata eventos
            events_loop.poll_events(|event| {
                use glutin::{Event, KeyboardInput, WindowEvent};
                // Limpa tela
                // Padrão é continuar o loop
                // Handling de eventos
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        // Em caso de evento de fechamento de tela, seta controle do loop de eventos para encerrar
                        WindowEvent::CloseRequested => should_break = true,
                        WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    virtual_keycode: Some(virtual_code),
                                    state,
                                    ..
                                },
                            ..
                        } => match (virtual_code, state) {
                            (glutin::VirtualKeyCode::Up, _) => {
                                (camera.update(camera.theta, camera.phi + 0.025, camera.distance));
                            }
                            (glutin::VirtualKeyCode::Down, _) => {
                                (camera.update(camera.theta, camera.phi - 0.025, camera.distance));
                            }
                            (glutin::VirtualKeyCode::Left, _) => {
                                (camera.update(camera.theta + 0.025, camera.phi, camera.distance));
                            }
                            (glutin::VirtualKeyCode::Right, _) => {
                                (camera.update(camera.theta - 0.025, camera.phi, camera.distance));
                            }
                            (glutin::VirtualKeyCode::End, _) => {
                                (camera.update(camera.theta, camera.phi, camera.distance + 0.025));
                            }
                            (glutin::VirtualKeyCode::Home, _) => {
                                (camera.update(camera.theta, camera.phi, camera.distance - 0.025));
                            }
                            (glutin::VirtualKeyCode::O, _) => is_view_orto = true,
                            (glutin::VirtualKeyCode::P, _) => is_view_orto = false,
                            _ => (),
                        },
                        _ => (),
                    },
                    _ => (),
                }
            });

            // Atualiza possiveis modificações de camera;
            view.update_camera(&camera);

            // Prepara view
            if is_view_orto {
                view.ortographic().render(&program);
            } else {
                view.render(&program);
            }

            // Desenha
            for i in 1..50 {
                cube.scale(1.0, 0.0005, 1.0)
                    .translate(0.0, 1.0, 0.0)
                    .scale((5.0 / i as f32).min(3.0), 1.0, 2.0)
                    .translate(0.0, i as f32 * 0.02 - 1.0, 0.0)
                    .draw(&program);
            }

            cow.translate(0f32, 0.7, 0f32)
                .draw(&program)
                .scale(0.5, 0.5, 0.5)
                .translate(0f32, 0f32, 0.75)
                .draw(&program)
                .translate(0f32, 0f32, -1.5)
                .draw(&program);

            //cube_big.draw(&program);
            //cube_small.draw(&program);
            gl_window.swap_buffers().unwrap();

            if should_break {
                break;
            }
        }
    }
}
