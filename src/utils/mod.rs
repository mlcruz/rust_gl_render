use gl::types::GLenum;
use gl::types::GLuint;

mod compile_shader;
mod link_program;
pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    compile_shader::compile_shader(src, ty)
}
pub fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    link_program::link_program(vs, fs)
}
