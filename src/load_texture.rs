#![allow(dead_code)]

use std::os::raw::c_void;
use std::path::Path;

use gl;
use image;
use image::GenericImage;

pub unsafe fn load_texture(path: &str) -> (u32, u32) {
    // Le arquivo de imagem
    let img = image::open(&Path::new(&path)).expect("Falha ao carregar textura");

    let data = img.raw_pixels();

    let mut texture_id = 0;
    let mut sampler_id = 0;

    gl::GenTextures(1, &mut texture_id);
    gl::GenSamplers(1, &mut sampler_id);

    // Veja slide 100 do documento "Aula_20_e_21_Mapeamento_de_Texturas.pdf"
    gl::SamplerParameteri(sampler_id, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
    gl::SamplerParameteri(sampler_id, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);

    // Parâmetros de amostragem da textura.
    gl::SamplerParameteri(
        sampler_id,
        gl::TEXTURE_MIN_FILTER,
        gl::LINEAR_MIPMAP_LINEAR as i32,
    );

    gl::SamplerParameteri(sampler_id, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
    gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
    gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 0);
    gl::PixelStorei(gl::UNPACK_SKIP_ROWS, 0);

    // Agora enviamos a imagem lida do disco para a GPU
    gl::ActiveTexture(gl::TEXTURE0 + texture_id);
    gl::BindTexture(gl::TEXTURE_2D, texture_id);

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::SRGB8 as i32,
        img.width() as i32,
        img.height() as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        &data[0] as *const u8 as *const c_void,
    );

    gl::GenerateMipmap(gl::TEXTURE_2D);
    gl::BindSampler(texture_id, sampler_id);

    (texture_id, sampler_id)
}
