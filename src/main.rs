extern crate gl;
extern crate glutin;
mod geo_objs;
mod utils;
use geo_objs::Cube;
use gl::types::*;
use glutin::dpi::LogicalSize;
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::path::Path;
use std::ptr;
use std::ptr::null;
use utils::compile_shader;
use utils::link_program;

// Vertex data
#[allow(dead_code)]
fn draw_cube(vao: &mut u32, vbo: &mut u32) -> () {
    // Geometria dos vertices de um cubo
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

    let vertex_data_size = (CUBE_VERTEX_GEOMETRY.len() * mem::size_of::<GLfloat>()) as GLsizeiptr;
    unsafe {
        // Definição dos atributos dos vertices
        // Cria identificador do de um VAO e "liga" o mesmo
        gl::GenVertexArrays(1, vao);
        gl::BindVertexArray(*vao);

        // Cria identificador de um VBO e "liga" o mesmo
        gl::GenBuffers(1, vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);

        // Alocamos memória para o VBO "ligado" acima.
        gl::BufferData(
            gl::ARRAY_BUFFER,
            vertex_data_size, // Tamanho dos vertices
            null(),
            gl::STATIC_DRAW,
        );
        // Copia valores dos array de vertices para o VBO
        gl::BufferSubData(
            gl::ARRAY_BUFFER,
            0,
            vertex_data_size,
            mem::transmute(&CUBE_VERTEX_GEOMETRY[0]),
        );

        // Precisamos então informar um índice de "local" ("location"), o qual será
        // utilizado no shader "shader_vertex.glsl" para acessar os valores
        // armazenados no VBO "ligado" acima.
        // Esta função também informa que o VBO "ligado" acima em glBindBuffer()
        // está dentro do VAO "ligado" acima por glBindVertexArray().
        // Veja https://www.khronos.org/opengl/wiki/Vertex_Specification#Vertex_Buffer_Object
        let location: GLuint = 0; // location 0 no vertex shader
        let number_of_dimensions: GLint = 4; // vec4 no shader
        let ptr_offset: *const std::ffi::c_void = 0 as *const std::ffi::c_void;
        gl::VertexAttribPointer(
            location,
            number_of_dimensions,
            gl::FLOAT,
            gl::FALSE,
            0,
            ptr_offset,
        );
        // "Ativamos" os atributos. Informamos que os atributos com índice de local
        // definido acima, na variável "location", deve ser utilizado durante o
        // rendering.
        gl::EnableVertexAttribArray(location);
        // Desliga VBO
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // Topologia:
    }
}

fn main() {
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
    let mut cube = Cube::new();

    events_loop.run_forever(|event| {
        use glutin::{ControlFlow, Event, WindowEvent};
        cube.draw(program);
        // Variavel de controle do loop de eventos
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
        unsafe {
            // Clear the screen to black
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        gl_window.swap_buffers().unwrap();

        control_flow_state
    });

    // Cleanup
    unsafe {
        gl::DeleteProgram(program);
        gl::DeleteShader(fs);
        gl::DeleteShader(vs);
    }
}
