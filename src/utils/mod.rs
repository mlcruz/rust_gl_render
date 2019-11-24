use gl::types::GLenum;
use gl::types::GLuint;

mod compile_shader;
mod link_program;
mod texture_from_file;

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    compile_shader::compile_shader(src, ty)
}
pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    link_program::link_program(vs, fs)
}

#[allow(dead_code)]
pub unsafe fn load_texture_from_file(path: &str, directory: &str) -> u32 {
    texture_from_file::TextureFromFile(path, directory)
}
