use game_loop::GameState;
use glutin::{DeviceEvent, Event, KeyboardInput, WindowEvent};
use models::scene_object::SceneObject;

use models::matrix::MatrixTransform;
use world::free_camera::FreeCamera;
use world::view::View;

// Trata possiveis entradas do usuario
pub fn handle_input(
    event: glutin::Event,
    game_state: &mut GameState,
    camera: &mut FreeCamera,
    _view: &mut View,
    speed: &mut f32,
    main_obj: &mut SceneObject,
) {
    match event {
        Event::WindowEvent { event, .. } => match event {
            // Em caso de evento de fechamento de tela, seta controle do loop de eventos para encerrar
            WindowEvent::CloseRequested => game_state.should_break = true,
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
                    camera.pos.z = camera.pos.z + (*speed * game_state.camera_speed_mult);
                }
                (glutin::VirtualKeyCode::Down, _) => {
                    // camera.update_angle(camera.theta, camera.phi - 0.025);
                    camera.pos.z = camera.pos.z - (*speed * game_state.camera_speed_mult);
                }
                (glutin::VirtualKeyCode::Left, _) => {
                    camera.pos.x = camera.pos.x + (*speed * game_state.camera_speed_mult);
                    // camera.update_angle(camera.theta + 0.025, camera.phi);
                }
                (glutin::VirtualKeyCode::Right, _) => {
                    camera.pos.x = camera.pos.x - (*speed * game_state.camera_speed_mult);
                    // camera.update_angle(camera.theta - 0.025, camera.phi);
                }
                (glutin::VirtualKeyCode::End, _) => {}
                (glutin::VirtualKeyCode::Home, _) => {}
                (glutin::VirtualKeyCode::O, _) => game_state.is_view_orto = true,
                (glutin::VirtualKeyCode::P, _) => game_state.is_view_orto = false,
                (glutin::VirtualKeyCode::W, _) => {
                    //  camera.pos.z = camera.pos.z - 0.01;
                    *main_obj = main_obj.translate(0.0, 0.0, *speed);
                }
                (glutin::VirtualKeyCode::S, _) => {
                    //camera.pos.z = camera.pos.z + 0.01;
                    *main_obj = main_obj.translate(0.0, 0.0, -*speed);
                }
                (glutin::VirtualKeyCode::A, _) => {
                    // camera.pos.x = camera.pos.x - 0.01;
                    *main_obj = main_obj.translate(*speed, 0.0, 0.00);
                }
                (glutin::VirtualKeyCode::D, _) => {
                    //camera.pos.x = camera.pos.x + 0.01;
                    *main_obj = main_obj.translate(-*speed, 0.0, 0.00);

                    // let mut new_pos =
                    //     normalize_vector(cross_product(camera.target, camera.up_vector)) * 0.01;

                    // if new_pos.x == 0.0 && new_pos.y == 0.0 && new_pos.z == 0.0 {
                    //     new_pos = glm::vec4(0.01, 0.0, 0.0, 0.0);
                    // }

                    // camera.update_position(&(camera.position + new_pos));
                }
                (glutin::VirtualKeyCode::Add, _) => {
                    game_state.score = game_state.score + 1;
                    game_state.should_add_obj = true;
                }
                (glutin::VirtualKeyCode::Minus, _) => {
                    game_state.score = game_state.score - 1;
                    game_state.should_add_obj = true;
                }
                _ => (),
            },
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
                let (xoffset, yoffset) = delta;
                let yaw = camera.yaw + (((xoffset as f64) * *speed as f64) as f32);
                let mut pitch = camera.pitch - (((yoffset as f64) * *speed as f64) as f32);

                let phimax = 3.141592 / 2.0;
                let phimin = -phimax;

                if pitch > phimax {
                    pitch = phimax;
                }

                if pitch < phimin {
                    pitch = phimin;
                }
                camera.pitch = pitch;
                camera.yaw = yaw;
            }
            _ => (),
        },
        _ => (),
    }
}
