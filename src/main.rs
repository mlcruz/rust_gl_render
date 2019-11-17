extern crate gl;
extern crate glm;
extern crate glutin;
mod camera;
mod geo_objs;
mod matrix;
mod shader_program;
mod utils;
mod view;
use camera::Camera;
use geo_objs::Cube;
use glutin::dpi::LogicalSize;
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

    // Compila e linka shaders
    let program = Shader::new("src/shader/vertex.glsl", "src/shader/fragment.glsl").program;

    // Inicializa camera
    let camera = Camera::new(g_camera_theta, g_camera_phi, g_camera_distance);

    // Inicializa matrizes de view e projeção com a camera criada
    let mut view = View::new(-0.1, -10.0, &camera);

    unsafe {
        gl::UseProgram(program);

        // Habilita Zbuffer
        gl::Enable(gl::DEPTH_TEST);

        // Inicializa um cubo
        let cube = Cube::new();
        let mut should_break = false;

        loop {
            events_loop.poll_events(|event| {
                use glutin::{Event, WindowEvent};
                // Limpa tela
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                gl::ClearColor(0.3, 0.3, 0.3, 1.0);
                // Padrão é continuar o loop
                // Handling de eventos
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        // Em caso de evento de fechamento de tela, seta controle do loop de eventos para encerrar
                        WindowEvent::CloseRequested => should_break = true,
                        _ => (),
                    },
                    _ => (),
                }

                view.update_camera(&camera);
                view.render(&program);
                cube.draw(&program);
                gl_window.swap_buffers().unwrap();
            });

            if should_break {
                break;
            }
        }
    }
}
