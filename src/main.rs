extern crate gl;
extern crate glm;
extern crate glutin;
extern crate image;
extern crate tobj;

mod models;
mod shader;
mod world;
use glutin::dpi::LogicalSize;
mod draw_frame;
mod handle_input;
use draw_frame::game_loop;

fn main() {
    // Inicializa loop de eventos da janela
    let mut events_loop = glutin::EventsLoop::new();

    // Iniciliza janela e contexto, com perfil core, versão 3.3, tamanho 800x600
    let window = glutin::WindowBuilder::new()
        .with_title("Rust Render")
        .with_dimensions(<LogicalSize>::new(1360f64, 768.0f64));

    let gl_window = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_gl_profile(glutin::GlProfile::Core)
        .build_windowed(window, &events_loop)
        .unwrap();

    // Coloca janela no contexto atual
    let gl_window = unsafe { gl_window.make_current() }.unwrap();
    gl_window.window().hide_cursor(true);

    // Carrega ponteiros para funções do openGL
    gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);

    unsafe {
        game_loop(&mut events_loop, &gl_window);
    }
}
