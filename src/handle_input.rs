use glutin::{DeviceEvent, Event, KeyboardInput, WindowEvent};
use models::matrix::cross_product;
use models::matrix::normalize_vector;
use world::camera::Camera;
use world::view::View;

// Trata possiveis entradas do usuario
pub fn handle_input(
    event: glutin::Event,
    should_break: &mut bool,
    is_view_orto: &mut bool,
    camera: &mut Camera,
    view: &mut View,
    speed: &mut f64,
) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            // Em caso de evento de fechamento de tela, seta controle do loop de eventos para encerrar
            WindowEvent::CloseRequested => *should_break = true,
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(virtual_code),
                        state,
                        ..
                    },
                ..
            } => match (virtual_code, state) {
                // Atualiza camera
                (glutin::VirtualKeyCode::Up, _) => {
                    camera.update_angle(camera.theta, camera.phi + 0.025);
                }
                (glutin::VirtualKeyCode::Down, _) => {
                    camera.update_angle(camera.theta, camera.phi - 0.025);
                }
                (glutin::VirtualKeyCode::Left, _) => {
                    camera.update_angle(camera.theta + 0.025, camera.phi);
                }
                (glutin::VirtualKeyCode::Right, _) => {
                    camera.update_angle(camera.theta - 0.025, camera.phi);
                }
                (glutin::VirtualKeyCode::End, _) => {}
                (glutin::VirtualKeyCode::Home, _) => {}
                (glutin::VirtualKeyCode::O, _) => *is_view_orto = true,
                (glutin::VirtualKeyCode::P, _) => *is_view_orto = false,
                (glutin::VirtualKeyCode::W, _) => {
                    camera.translate_position(&glm::vec4(0.00, 0.0, 0.01, 0.0));
                }
                (glutin::VirtualKeyCode::S, _) => {
                    camera.translate_position(&glm::vec4(0.00, 0.0, -0.01, 0.0));
                }
                (glutin::VirtualKeyCode::A, _) => {
                    let mut new_pos =
                        normalize_vector(cross_product(camera.target, camera.up_vector)) * 0.01;

                    if new_pos.x == 0.0 && new_pos.y == 0.0 && new_pos.z == 0.0 {
                        new_pos = glm::vec4(0.01, 0.0, 0.0, 0.0);
                    }
                    camera.update_position(&(camera.position - new_pos));
                }
                (glutin::VirtualKeyCode::D, _) => {
                    let mut new_pos =
                        normalize_vector(cross_product(camera.target, camera.up_vector)) * 0.01;

                    if new_pos.x == 0.0 && new_pos.y == 0.0 && new_pos.z == 0.0 {
                        new_pos = glm::vec4(0.01, 0.0, 0.0, 0.0);
                    }

                    camera.update_position(&(camera.position + new_pos));
                }
                _ => (),
            },
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
                let (xoffset, yoffset) = delta;
                let theta = camera.theta + (((xoffset as f64) * *speed) as f32);
                let mut phi = camera.phi + (((yoffset as f64) * *speed) as f32);

                let phimax = 3.141592 / 2.0;
                let phimin = -phimax;

                if phi > phimax {
                    phi = phimax;
                }

                if phi < phimin {
                    phi = phimin;
                }
                camera.update_angle(theta, phi);
                println!("{:?}", camera);
            }
            _ => (),
        },
        _ => (),
    }
}
