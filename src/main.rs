extern crate gl;
extern crate glm;
extern crate glutin;
extern crate image;
extern crate tobj;

mod models;
mod shader;
mod world;
use glutin::dpi::LogicalSize;
use models::draw::Draw;
use models::matrix::MatrixTransform;
use models::scene_object::SceneObject;
use shader::shader_program::Shader;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use world::camera::Camera;
use world::view::View;

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

    // Compila e linka shaders
    let program = Shader::new(
        "src/data/shader/vertex.glsl",
        "src/data/shader/fragment.glsl",
    )
    .program;

    // Inicializa camera
    let mut camera = Camera::new(g_camera_theta, g_camera_phi, g_camera_distance);

    // Inicializa matrizes de view e projeção com a camera criada
    let mut view = View::new(-0.01, -10.0, &camera);
    let mut is_view_orto = false;
    let mut framerate = 120.0;
    unsafe {
        gl::UseProgram(program);

        // Habilita Zbuffer
        gl::Enable(gl::DEPTH_TEST);

        // Inicializa uma vaca
        let cow = SceneObject::new("src/data/objs/cow.obj").scale(0.5, 0.5, 0.5);
        let bunny = SceneObject::new("src/data/objs/bunny.obj")
            .translate(1.0, 1.0, 1.0)
            .scale(0.5, 0.5, 0.5)
            .load_texture("src/data/textures/tc-earth_daymap_surface.jpg")
            .with_specular_phong_q(&16.0)
            .with_specular_reflectance(&glm::vec3(0.65, 0.5, 0.5));

        let blinking_cow = cow
            .load_texture("src/data/textures/tc-earth_nightmap_citylights.gif")
            .with_specular_reflectance(&glm::vec3(1.0, 1.0, 1.0))
            .translate(0.5, 0.5, 0.5);

        let night_cow = cow
            .load_texture("src/data/textures/tc-earth_daymap_surface.jpg")
            .with_specular_reflectance(&glm::vec3(0.8, 0.8, 0.8))
            .with_specular_phong_q(&12.0)
            .translate(-0.5, -0.5, -0.5);

        let the_horror = bunny.add_children(&blinking_cow);

        let mut should_break = false;

        // timing
        let mut delta_time: f32 = 0.001;

        loop {
            let timer = Instant::now();
            let relative_frametime = delta_time * framerate;

            // Mov dos objetos por segundo ()
            // let speed = delta_time * 0.8;

            // Mov dos objs por frame
            let speed = relative_frametime * 0.01;
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);

            // Trata eventos
            events_loop.poll_events(|event| {
                use glutin::{DeviceEvent, Event, KeyboardInput, WindowEvent};
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
                                camera.update_angle(camera.theta, camera.phi + 0.025);
                            }
                            (glutin::VirtualKeyCode::Down, _) => {
                                //(camera.update(camera.theta, camera.phi - 0.025, camera.distance));
                                camera.update_angle(camera.theta, camera.phi - 0.025);
                            }
                            (glutin::VirtualKeyCode::Left, _) => {
                                //(camera.update(camera.theta + 0.025, camera.phi, camera.distance));
                                camera.update_angle(camera.theta + 0.025, camera.phi);
                            }
                            (glutin::VirtualKeyCode::Right, _) => {
                                //(camera.update(camera.theta - 0.025, camera.phi, camera.distance));
                                camera.update_angle(camera.theta - 0.025, camera.phi);
                            }
                            (glutin::VirtualKeyCode::End, _) => {
                                //(camera.update(camera.theta, camera.phi, camera.distance + 0.025));
                                // camera.distance = camera.distance + 0.025;
                            }
                            (glutin::VirtualKeyCode::Home, _) => {
                                //(camera.update(camera.theta, camera.phi, camera.distance - 0.025));
                            }
                            (glutin::VirtualKeyCode::O, _) => is_view_orto = true,
                            (glutin::VirtualKeyCode::P, _) => is_view_orto = false,
                            (glutin::VirtualKeyCode::W, _) => {
                                //view.translate_position(0.05, 0.0, 0.0);
                                camera.translate_position(&glm::vec4(0.00, 0.0, -0.01, 0.0));
                            }
                            (glutin::VirtualKeyCode::A, _) => {
                                //view.translate_position(0.05, 0.0, 0.0);
                                camera.translate_position(&glm::vec4(-0.01, 0.0, 0.0, 0.0));
                            }
                            (glutin::VirtualKeyCode::S, _) => {
                                //view.translate_position(0.05, 0.0, 0.0);
                                camera.translate_position(&glm::vec4(0.00, 0.0, 0.01, 0.0));
                            }
                            (glutin::VirtualKeyCode::D, _) => {
                                //view.translate_position(0.05, 0.0, 0.0);
                                camera.translate_position(&glm::vec4(0.01, 0.0, 0.00, 0.0));
                            }
                            _ => (),
                        },
                        _ => (),
                    },
                    Event::DeviceEvent { event, .. } => match event {
                        DeviceEvent::MouseMotion { delta } => {
                            let (xoffset, yoffset) = delta;

                            let theta = camera.theta + (xoffset as f32) * speed;
                            let mut phi = camera.phi + (yoffset as f32) * speed;

                            // Em coordenadas esféricas, o ângulo phi deve ficar entre -pi/2 e +pi/2.
                            let phimax = 3.141592 / 2.0;
                            let phimin = -phimax;

                            if phi > phimax {
                                phi = phimax;
                            }

                            if phi < phimin {
                                phi = phimin;
                            }

                            camera.update_angle(theta, phi);
                        }
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

            cow.draw(&program);
            blinking_cow.draw(&program);
            night_cow.draw(&program);
            the_horror.draw(&program);

            delta_time = timer.elapsed().as_secs_f32();

            sleep(Duration::from_secs_f32(glm::max(
                (1.0 / framerate) - delta_time,
                0.0,
            )));
            delta_time = timer.elapsed().as_secs_f32();
            gl_window.swap_buffers().unwrap();

            if should_break {
                break;
            }
        }
    }
}
