use game_loop::gen_random_usize;
use game_loop::GameState;
use glutin::{DeviceEvent, Event, KeyboardInput, WindowEvent};
use models::matrix::cross_product;
use models::matrix::normalize_vector;
use models::scene_object::SceneObject;
use world::lighting::Lighting;

use models::matrix::MatrixTransform;
use world::free_camera::FreeCamera;
use world::view::View;

// Trata possiveis entradas do usuario
pub fn handle_input(
    event: glutin::Event,
    game_state: &mut GameState,
    look_at_camera: &mut FreeCamera,
    free_camera: &mut FreeCamera,
    view: &mut View,
    speed: &mut f32,
    main_obj: &mut SceneObject,
    plane: &mut SceneObject,
    texture_pool: &Vec<&u32>,
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
                // Trata input do usuario
                (glutin::VirtualKeyCode::Up, glutin::ElementState::Pressed) => {
                    look_at_camera.pos.z =
                        look_at_camera.pos.z + (*speed * game_state.camera_speed_mult);

                    if game_state.lighting_source != glm::vec4(0.0, 0.0, 0.0, 0.0) {
                        game_state.lighting_source.z = game_state.lighting_source.z + 0.05;
                    }
                }
                (glutin::VirtualKeyCode::Down, glutin::ElementState::Pressed) => {
                    look_at_camera.pos.z =
                        look_at_camera.pos.z - (*speed * game_state.camera_speed_mult);

                    if game_state.lighting_source != glm::vec4(0.0, 0.0, 0.0, 0.0) {
                        game_state.lighting_source.z = game_state.lighting_source.z - 0.05;
                    }
                }
                (glutin::VirtualKeyCode::Left, glutin::ElementState::Pressed) => {
                    look_at_camera.pos.x =
                        look_at_camera.pos.x + (*speed * game_state.camera_speed_mult);

                    if game_state.lighting_source != glm::vec4(0.0, 0.0, 0.0, 0.0) {
                        game_state.lighting_source.x = game_state.lighting_source.x + 0.05;
                    }
                }
                (glutin::VirtualKeyCode::Right, glutin::ElementState::Pressed) => {
                    look_at_camera.pos.x =
                        look_at_camera.pos.x - (*speed * game_state.camera_speed_mult);

                    if game_state.lighting_source != glm::vec4(0.0, 0.0, 0.0, 0.0) {
                        game_state.lighting_source.x = game_state.lighting_source.x - 0.05;
                    }
                }
                (glutin::VirtualKeyCode::Insert, glutin::ElementState::Pressed) => {
                    if game_state.lighting_source != glm::vec4(0.0, 0.0, 0.0, 0.0) {
                        game_state.lighting_source.y = game_state.lighting_source.y + 0.05;
                    }
                }
                (glutin::VirtualKeyCode::Delete, glutin::ElementState::Pressed) => {
                    if game_state.lighting_source != glm::vec4(0.0, 0.0, 0.0, 0.0) {
                        game_state.lighting_source.y = game_state.lighting_source.y - 0.05;
                    }
                }
                (glutin::VirtualKeyCode::R, glutin::ElementState::Pressed) => {
                    // Reseta camera para a posição inicial
                    view.lighting = Lighting::new(
                        &glm::vec3(1.0, 1.0, 1.0),
                        &glm::vec3(0.25, 0.25, 0.25),
                        &glm::vec4(1.0, 1.0, 0.0, 0.0),
                    );

                    // Recarrega texturas
                    let rand_intp = gen_random_usize() % texture_pool.len();
                    *plane = plane.with_texture(&texture_pool.as_slice()[rand_intp], 2);
                    *main_obj = main_obj.with_texture(&texture_pool.as_slice()[rand_intp], 1);
                    game_state.lighting_source = glm::vec4(0.0, -18.0, 0.0, 1.0);
                }
                (glutin::VirtualKeyCode::B, glutin::ElementState::Pressed) => {
                    game_state.with_bezier = !game_state.with_bezier;
                }

                (glutin::VirtualKeyCode::Numpad7, glutin::ElementState::Pressed) => {
                    view.lighting.global = glm::vec3(
                        view.lighting.global.x + 0.05,
                        view.lighting.global.y,
                        view.lighting.global.z,
                    );
                }
                (glutin::VirtualKeyCode::Numpad8, glutin::ElementState::Pressed) => {
                    view.lighting.global = glm::vec3(
                        view.lighting.global.x,
                        view.lighting.global.y + 0.05,
                        view.lighting.global.z,
                    );
                }
                (glutin::VirtualKeyCode::Numpad9, glutin::ElementState::Pressed) => {
                    view.lighting.global = glm::vec3(
                        view.lighting.global.x,
                        view.lighting.global.y,
                        view.lighting.global.z + 0.05,
                    );
                }
                (glutin::VirtualKeyCode::Numpad1, glutin::ElementState::Pressed) => {
                    view.lighting.global = glm::vec3(
                        view.lighting.global.x - 0.05,
                        view.lighting.global.y,
                        view.lighting.global.z,
                    );
                }
                (glutin::VirtualKeyCode::Numpad2, glutin::ElementState::Pressed) => {
                    view.lighting.global = glm::vec3(
                        view.lighting.global.x,
                        view.lighting.global.y - 0.05,
                        view.lighting.global.z,
                    );
                }
                (glutin::VirtualKeyCode::Numpad3, glutin::ElementState::Pressed) => {
                    view.lighting.global = glm::vec3(
                        view.lighting.global.x,
                        view.lighting.global.y,
                        view.lighting.global.z - 0.05,
                    );
                }
                (glutin::VirtualKeyCode::F1, glutin::ElementState::Pressed) => {
                    view.lighting.global_direction = glm::vec4(
                        view.lighting.global_direction.x - 0.05,
                        view.lighting.global_direction.y,
                        view.lighting.global_direction.z,
                        view.lighting.global_direction.w,
                    );
                }
                (glutin::VirtualKeyCode::F2, glutin::ElementState::Pressed) => {
                    view.lighting.global_direction = glm::vec4(
                        view.lighting.global_direction.x,
                        view.lighting.global_direction.y - 0.05,
                        view.lighting.global_direction.z,
                        view.lighting.global_direction.w,
                    );
                }
                (glutin::VirtualKeyCode::F3, glutin::ElementState::Pressed) => {
                    view.lighting.global_direction = glm::vec4(
                        view.lighting.global_direction.x,
                        view.lighting.global_direction.y,
                        view.lighting.global_direction.z - 0.05,
                        view.lighting.global_direction.w,
                    );
                }
                (glutin::VirtualKeyCode::F7, glutin::ElementState::Pressed) => {
                    view.lighting.global_direction = glm::vec4(
                        view.lighting.global_direction.x + 0.05,
                        view.lighting.global_direction.y,
                        view.lighting.global_direction.z,
                        view.lighting.global_direction.w,
                    );
                }
                (glutin::VirtualKeyCode::F8, glutin::ElementState::Pressed) => {
                    view.lighting.global_direction = glm::vec4(
                        view.lighting.global_direction.x,
                        view.lighting.global_direction.y + 0.05,
                        view.lighting.global_direction.z,
                        view.lighting.global_direction.w,
                    );
                }
                (glutin::VirtualKeyCode::F9, glutin::ElementState::Pressed) => {
                    view.lighting.global_direction = glm::vec4(
                        view.lighting.global_direction.x,
                        view.lighting.global_direction.y,
                        view.lighting.global_direction.z + 0.05,
                        view.lighting.global_direction.w,
                    );
                }
                (glutin::VirtualKeyCode::Key1, glutin::ElementState::Pressed) => {
                    view.lighting.ambient = glm::vec3(
                        view.lighting.ambient.x - 0.05,
                        view.lighting.ambient.y,
                        view.lighting.ambient.z,
                    );
                }
                (glutin::VirtualKeyCode::Key2, glutin::ElementState::Pressed) => {
                    view.lighting.ambient = glm::vec3(
                        view.lighting.ambient.x,
                        view.lighting.ambient.y - 0.05,
                        view.lighting.ambient.z,
                    );
                }
                (glutin::VirtualKeyCode::Key3, glutin::ElementState::Pressed) => {
                    view.lighting.ambient = glm::vec3(
                        view.lighting.ambient.x,
                        view.lighting.ambient.y,
                        view.lighting.ambient.z - 0.05,
                    );
                }
                (glutin::VirtualKeyCode::Key7, glutin::ElementState::Pressed) => {
                    view.lighting.ambient = glm::vec3(
                        view.lighting.ambient.x + 0.05,
                        view.lighting.ambient.y,
                        view.lighting.ambient.z,
                    );
                }
                (glutin::VirtualKeyCode::Key8, glutin::ElementState::Pressed) => {
                    view.lighting.ambient = glm::vec3(
                        view.lighting.ambient.x,
                        view.lighting.ambient.y + 0.05,
                        view.lighting.ambient.z,
                    );
                }
                (glutin::VirtualKeyCode::Key9, glutin::ElementState::Pressed) => {
                    view.lighting.ambient = glm::vec3(
                        view.lighting.ambient.x,
                        view.lighting.ambient.y,
                        view.lighting.ambient.z + 0.05,
                    );
                }
                (glutin::VirtualKeyCode::Home, glutin::ElementState::Pressed) => {
                    view.lighting.global = glm::vec3(
                        view.lighting.global.x + 0.05,
                        view.lighting.global.y + 0.05,
                        view.lighting.global.z + 0.05,
                    );
                }
                (glutin::VirtualKeyCode::End, glutin::ElementState::Pressed) => {
                    view.lighting.global = glm::vec3(
                        view.lighting.global.x - 0.05,
                        view.lighting.global.y - 0.05,
                        view.lighting.global.z - 0.05,
                    )
                }
                (glutin::VirtualKeyCode::PageDown, glutin::ElementState::Pressed) => {
                    view.lighting.ambient = glm::vec3(
                        view.lighting.ambient.x - 0.05,
                        view.lighting.ambient.y - 0.05,
                        view.lighting.ambient.z - 0.05,
                    )
                }
                (glutin::VirtualKeyCode::PageUp, glutin::ElementState::Pressed) => {
                    view.lighting.ambient = glm::vec3(
                        view.lighting.ambient.x + 0.05,
                        view.lighting.ambient.y + 0.05,
                        view.lighting.ambient.z + 0.05,
                    )
                }
                (glutin::VirtualKeyCode::O, glutin::ElementState::Pressed) => {
                    game_state.is_view_orto = true
                }
                (glutin::VirtualKeyCode::P, glutin::ElementState::Pressed) => {
                    game_state.is_view_orto = false
                }
                (glutin::VirtualKeyCode::W, _) => {
                    // Atualiza obj para mover em direção ao vetor de direção frontal da camera
                    if game_state.current_camera == 1 {
                        let mut new_vec = normalize_vector(free_camera.front) * *speed;

                        // Permite movimentação no plano y
                        if game_state.can_fly == false {
                            new_vec.y = 0.0;
                        }
                        *main_obj = main_obj.translate(new_vec.x, new_vec.y, new_vec.z)
                    } else {
                        *main_obj = main_obj.translate(0.0, 0.0, *speed);
                    }
                }
                (glutin::VirtualKeyCode::S, _) => {
                    if game_state.current_camera == 1 {
                        let mut new_vec = normalize_vector(free_camera.front) * *speed;
                        if game_state.can_fly == false {
                            new_vec.y = 0.0;
                        }
                        *main_obj = main_obj.translate(-new_vec.x, -new_vec.y, -new_vec.z)
                    } else {
                        *main_obj = main_obj.translate(0.0, 0.0, -*speed);
                    }
                }
                (glutin::VirtualKeyCode::A, _) => {
                    if game_state.current_camera == 1 {
                        // Move na direção do vetor normal ao vetor de direçao
                        let new_vec = normalize_vector(cross_product(
                            free_camera.front,
                            glm::vec4(0.0, 1.0, 0.0, 0.0),
                        )) * *speed;
                        *main_obj = main_obj.translate(-new_vec.x, -new_vec.y, -new_vec.z);
                    } else {
                        *main_obj = main_obj.translate(*speed, 0.0, 0.00);
                    }
                }
                (glutin::VirtualKeyCode::D, _) => {
                    if game_state.current_camera == 1 {
                        let new_vec = normalize_vector(cross_product(
                            free_camera.front,
                            glm::vec4(0.0, 1.0, 0.0, 0.0),
                        )) * *speed;
                        *main_obj = main_obj.translate(new_vec.x, new_vec.y, new_vec.z);
                    } else {
                        *main_obj = main_obj.translate(-*speed, 0.0, 0.00);
                    }
                }
                (glutin::VirtualKeyCode::Add, glutin::ElementState::Pressed) => {
                    game_state.score = game_state.score + 1;
                    game_state.should_add_obj = true;
                }
                (glutin::VirtualKeyCode::Equals, glutin::ElementState::Pressed) => {
                    game_state.score = game_state.score + 1;
                    game_state.should_add_obj = true;
                }
                (glutin::VirtualKeyCode::Subtract, glutin::ElementState::Pressed) => {
                    game_state.score = game_state.score - 1;
                    game_state.should_add_obj = true;
                    game_state.draw_queue = Vec::new();
                }
                (glutin::VirtualKeyCode::Minus, glutin::ElementState::Pressed) => {
                    game_state.score = game_state.score - 1;
                    game_state.should_add_obj = true;
                    game_state.draw_queue = Vec::new();
                }
                _ => (), // _ => println!("{:?} {:?}", virtual_code, state),
            },
            _ => (),
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::MouseMotion { delta } => {
                // Trata movimentação do angula da camera utilizando o mouse
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

    if view.lighting.ambient.x < 0.0 {
        view.lighting.ambient.x = 0.0
    }
    if view.lighting.ambient.y < 0.0 {
        view.lighting.ambient.y = 0.0
    }
    if view.lighting.ambient.z < 0.0 {
        view.lighting.ambient.z = 0.0
    }

    if view.lighting.global.x < 0.0 {
        view.lighting.global.x = 0.0
    }
    if view.lighting.global.y < 0.0 {
        view.lighting.global.y = 0.0
    }
    if view.lighting.global.z < 0.0 {
        view.lighting.global.z = 0.0
    }
}
