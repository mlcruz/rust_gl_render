use shader::compile_shader::compile_shader;
use shader::link_program::link_program;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct Shader {
    vertex_shader: String,
    fragment_shader: String,
    vs: u32,
    fs: u32,
    pub program: u32,
}

#[allow(dead_code)]
impl Shader {
    pub fn new(vertex_location: &str, fragment_location: &str) -> Self {
        let vertex_path = Path::new(vertex_location);
        let fragment_path = Path::new(fragment_location);

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

        // Compila shaders e linka shaders
        let vs = compile_shader(&vertex_shader, gl::VERTEX_SHADER);
        let fs = compile_shader(&fragment_shader, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);

        Shader {
            vertex_shader,
            fragment_shader,
            vs,
            fs,
            program,
        }
    }
}
