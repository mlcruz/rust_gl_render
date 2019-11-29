use game_loop::GameState;
use glutin::{DeviceEvent, Event, KeyboardInput, WindowEvent};
use models::matrix::cross_product;
use models::matrix::normalize_vector;
use models::scene_object::SceneObject;

use models::matrix::MatrixTransform;
use world::free_camera::FreeCamera;
use world::view::View;

// Trata possiveis entradas do usuario
pub fn handle_input(
    event: glutin::Event,
    game_state: &mut GameState,
    look_at_camera: &mut FreeCamera,
    free_camera: &mut FreeCamera,
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
                    look_at_camera.pos.z =
                        look_at_camera.pos.z + (*speed * game_state.camera_speed_mult);
                }
                (glutin::VirtualKeyCode::Down, _) => {
                    // camera.update_angle(camera.theta, camera.phi - 0.025);
                    look_at_camera.pos.z =
                        look_at_camera.pos.z - (*speed * game_state.camera_speed_mult);
                }
                (glutin::VirtualKeyCode::Left, _) => {
                    look_at_camera.pos.x =
                        look_at_camera.pos.x + (*speed * game_state.camera_speed_mult);
                    // camera.update_angle(camera.theta + 0.025, camera.phi);
                }
                (glutin::VirtualKeyCode::Right, _) => {
                    look_at_camera.pos.x =
                        look_at_camera.pos.x - (*speed * game_state.camera_speed_mult);
                    // camera.update_angle(camera.theta - 0.025, camera.phi);
                }
                (glutin::VirtualKeyCode::End, _) => {}
                (glutin::VirtualKeyCode::Home, _) => {}
                (glutin::VirtualKeyCode::O, _) => game_state.is_view_orto = true,
                (glutin::VirtualKeyCode::P, _) => game_state.is_view_orto = false,
                (glutin::VirtualKeyCode::W, _) => {
                    if game_state.current_camera == 1 {
                        *main_obj = main_obj.translate(*speed, 0.0, 0.0);
                    } else {
                        *main_obj = main_obj.translate(0.0, 0.0, *speed);
                    }
                }
                (glutin::VirtualKeyCode::S, _) => {
                    //camera.pos.z = camera.pos.z + 0.01;
                    if game_state.current_camera == 1 {
                        *main_obj = main_obj.translate(-*speed, 0.0, 0.0);
                    } else {
                        *main_obj = main_obj.translate(0.0, 0.0, -*speed);
                    }
                }
                (glutin::VirtualKeyCode::A, _) => {
                    if game_state.current_camera == 1 {
                        let new_vew = normalize_vector(cross_product(
                            free_camera.front,
                            glm::vec4(0.0, 1.0, 0.0, 0.0),
                        )) * *speed;
                        *main_obj = main_obj.translate(-new_vew.x, -new_vew.y, -new_vew.z);
                    } else {
                        *main_obj = main_obj.translate(*speed, 0.0, 0.00);
                    }
                }
                (glutin::VirtualKeyCode::D, _) => {
                    //camera.pos.x = camera.pos.x + 0.01;

                    if game_state.current_camera == 1 {
                        let new_vew = normalize_vector(cross_product(
                            free_camera.front,
                            glm::vec4(0.0, 1.0, 0.0, 0.0),
                        )) * *speed;
                        *main_obj = main_obj.translate(new_vew.x, new_vew.y, new_vew.z);
                    } else {
                        *main_obj = main_obj.translate(-*speed, 0.0, 0.00);
                    }
                }
                (glutin::VirtualKeyCode::Add, glutin::ElementState::Pressed) => {
                    game_state.score = game_state.score + 1;
                    game_state.should_add_obj = true;
                    println!("{:?}", game_state.score);
                }
                (glutin::VirtualKeyCode::Equals, glutin::ElementState::Pressed) => {
                    game_state.score = game_state.score + 1;
                    game_state.should_add_obj = true;
                    println!("{:?}", game_state.score);
                }
                (glutin::VirtualKeyCode::Subtract, glutin::ElementState::Pressed) => {
                    game_state.score = game_state.score - 1;
                    game_state.should_add_obj = true;
                    game_state.draw_queue = Vec::new();
                    println!("{:?}", game_state.score);
                }
                (glutin::VirtualKeyCode::Minus, glutin::ElementState::Pressed) => {
                    game_state.score = game_state.score - 1;
                    game_state.should_add_obj = true;
                    game_state.draw_queue = Vec::new();
                    println!("{:?}", game_state.score);
                }
                _ => (), // _ => println!("{:?} {:?}", virtual_code, state),
            },
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
                if game_state.current_camera == 1 {
                    let (xoffset, yoffset) = delta;
                    let yaw = free_camera.yaw + (((xoffset as f64) * (*speed / 3.0) as f64) as f32);
                    let mut pitch =
                        free_camera.pitch - (((yoffset as f64) * (*speed / 3.0) as f64) as f32);

                    let phimax = 3.141592 / 2.0;
                    let phimin = -phimax;

                    if pitch > phimax {
                        pitch = phimax;
                    }

                    if pitch < phimin {
                        pitch = phimin;
                    }
                    free_camera.pitch = pitch;
                    free_camera.yaw = yaw;
                }
            }
            _ => (),
        },
        _ => (),
    }
}
