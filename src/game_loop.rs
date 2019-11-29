use handle_input::handle_input;
use models::draw::Draw;
use models::load_texture::load_texture;
use models::matrix::normalize_vector;
use models::matrix::MatrixTransform;
use models::scene_object::SceneObject;
use shader::shader_program::Shader;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use world::free_camera::FreeCamera;
use world::view::View;

// Controle do loop principal do jogo

// Controle de estado do jogo
#[allow(dead_code, unused_assignments)]
pub struct GameState {
    pub should_break: bool,
    pub should_add_obj: bool,
    pub is_view_orto: bool,
    pub draw_queue: Vec<SceneObject>,
    pub score: i32,
    pub framerate: i32,
    pub camera_height: f32,
    pub obj_plane_height: f32,
    pub speed_mult: f64,
    pub look_at: glm::Vec4,
    pub camera_speed_mult: f32,
    pub current_camera: i32,
}

#[allow(dead_code, unused_assignments)]
pub unsafe fn game_loop(
    events_loop: &mut glutin::EventsLoop,
    gl_window: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>,
) {
    // Compila e linka shaders
    let default_shader = Shader::new(
        "src/data/shader/vertex/default.glsl",
        "src/data/shader/fragment/default.glsl",
    )
    .program;

    // Compila e linka shaders
    let lambert_illumination = Shader::new(
        "src/data/shader/vertex/default.glsl",
        "src/data/shader/fragment/lambert_ilumination.glsl",
    )
    .program;

    let phong_illumination = Shader::new(
        "src/data/shader/vertex/default.glsl",
        "src/data/shader/fragment/phong_ilumination.glsl",
    )
    .program;
    gl::Enable(gl::DEPTH_TEST);

    // Inicializa estado do jogo
    let mut game_state = GameState {
        is_view_orto: true,
        should_break: false,
        should_add_obj: true,
        draw_queue: Vec::new(),
        framerate: 120,
        score: 0,
        camera_height: 0.0,
        obj_plane_height: -20.0,
        speed_mult: 3.0,
        camera_speed_mult: 0.0,
        look_at: glm::vec4(0.0, -1.0, 0.000000000001, 0.0),
        current_camera: 0,
    };

    let (sad_texture, _) = load_texture("src/data/textures/sad.jpg");
    let (grass_texture, _) = load_texture("src/data/textures/grass.jpg");
    let (fabric_texture, _) = load_texture("src/data/textures/fabric.jpg");
    let (pearl_texture, _) = load_texture("src/data/textures/pearl.jpg");

    let texture_pool = vec![&grass_texture, &fabric_texture, &pearl_texture];
    let plane_pool = vec![&pearl_texture, &fabric_texture];
    let mut plane = SceneObject::new("src/data/objs/plane.obj")
        .scale(5.0, 5.0, 5.0)
        .translate(0.0, game_state.obj_plane_height, 0.0)
        .with_color(&glm::vec3(0.6, 0.6, 0.6));

    let mut sad_plane = SceneObject::new("src/data/objs/plane.obj")
        .scale(1.0, 1.0, 1.0)
        .translate(0.0, game_state.obj_plane_height + 0.001, 0.0)
        .with_color(&glm::vec3(0.6, 0.6, 0.6));

    let sad_head =
        SceneObject::new("src/data/objs/sphere.obj").with_color(&glm::vec3(0.0, 0.0, 0.0));
    // Inicializa objetos do cenario

    let mut main_obj = SceneObject::new("src/data/objs/cube.obj")
        .with_color(&glm::vec3(1.0, 1.0, 1.0))
        .scale(0.2, 0.2, 0.2)
        .translate(0.0, game_state.obj_plane_height, -0.0);

    let base_cube = SceneObject::new("src/data/objs/cube.obj");
    let framerate = 120.0;

    let mut current_shader = &default_shader;

    // Inicializa camera livre
    let mut free_camera =
        FreeCamera::new(glm::vec3(0.0, game_state.camera_height, 0.0), &0.0, &0.0);

    // Inicializa camera look at e define vetor fixo
    let mut look_at_camera =
        FreeCamera::new(glm::vec3(0.0, game_state.camera_height, 0.0), &0.0, &0.0);

    look_at_camera.front = game_state.look_at;
    free_camera.front = game_state.look_at;

    // Inicializa camera
    // Inicializa matriz de projeção com a camera criada
    let mut view = View::new(-0.01, -20.0, &look_at_camera);

    // Contador de tempo de frame
    let mut delta_time: f64 = 0.001;

    // Controle de velocidade
    let mut speed = 0.0f64;

    let mut move_camera = false;

    loop {
        // Inicializa cronometro de tempo de renderização de uma frame
        let timer = Instant::now();

        // speed_mult unidades por segundo
        speed = delta_time * game_state.speed_mult;

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);

        // Trata eventos
        events_loop.poll_events(|event| {
            handle_input(
                event,
                &mut game_state,
                &mut look_at_camera,
                &mut free_camera,
                &mut view,
                &mut (speed as f32),
                &mut main_obj,
            );
        });
        // Gera uma alteração de estado do loop do jogo
        if game_state.should_add_obj {
            let mut new_obj1 = generate_random_obj(&base_cube, game_state.obj_plane_height);
            let mut new_obj2 = generate_random_obj(&base_cube, game_state.obj_plane_height);
            let mut new_obj3 = generate_random_obj(&base_cube, game_state.obj_plane_height);
            move_camera = false;

            // Posiveis alterações feitas em iterações anteriores são revertidas
            game_state.look_at = glm::vec4(0.0, -1.0, 0.000000000001, 0.0);
            game_state.camera_height = 0.0;
            game_state.is_view_orto = true;
            current_shader = &default_shader;

            game_state.current_camera = 0;

            if game_state.score >= 2 {
                new_obj1 = new_obj1.with_color(&gen_random_vec3());
                new_obj2 = new_obj2.with_color(&gen_random_vec3());
                new_obj3 = new_obj3.with_color(&gen_random_vec3());
            }

            if game_state.score == 2 {
                println!("Cor!")
            }
            if game_state.score == 4 {
                main_obj = main_obj.with_color(&gen_random_vec3());
                let rand_plane_color = gen_random_vec3();

                plane = plane.with_color(&rand_plane_color);
                sad_plane = sad_plane.with_color(&rand_plane_color);
            }
            if game_state.score == 5 {
                look_at_camera.pos.z = 0.0;
            }

            if game_state.score >= 6 {
                game_state.is_view_orto = false;
                if look_at_camera.pos.z == 0.0
                    && look_at_camera.pos.x == 0.0
                    && game_state.camera_height == 0.0
                {
                    look_at_camera.pos.z = -6.0;
                    free_camera.pos.z = -6.0;
                }
                game_state.camera_height = -17.0;
                game_state.look_at = glm::vec4(0.0, -0.35, 1.0, 0.0);
                game_state.camera_speed_mult = 1.0;
            }
            if game_state.score == 6 {
                println!("Proj. Perpesctiva e camera movel!");
            }
            if game_state.score >= 8 {
                current_shader = &lambert_illumination;
                let rand_int = gen_random_usize() % texture_pool.len();

                main_obj = main_obj.add_children(
                    &sad_head
                        .with_texture(&texture_pool.as_slice()[rand_int], 4)
                        .scale(0.35, 0.35, 0.35)
                        .translate(0.5, 1.5, -0.0),
                );
            }
            if game_state.score == 8 {
                println!("Iluminação de lambert!")
            }

            if game_state.score >= 10 {
                main_obj = main_obj.with_color(&glm::vec3(0.0, 0.0, 0.0));

                // Textutas aleatorioas
                let rand_int = gen_random_usize() % texture_pool.len();
                let rand_int3 = gen_random_usize() % texture_pool.len();
                let rand_int4 = gen_random_usize() % texture_pool.len();
                let rand_int5 = gen_random_usize() % texture_pool.len();

                let rand_int2 = gen_random_usize() % plane_pool.len();

                // Tipos Mapeamentos de textura aleatorios
                let rand_int7 = (gen_random_i32() % 5) + 1;
                let rand_int8 = (gen_random_i32() % 5) + 1;

                if main_obj.get_texture_override() == 0 {
                    main_obj = main_obj.with_texture(&texture_pool.as_slice()[rand_int], 1);
                }

                if plane.get_texture_override() == 0 {
                    plane = plane.with_color(&glm::vec3(0.0, 0.0, 0.0));
                    plane = plane.with_texture(&plane_pool.as_slice()[rand_int2], 2);
                    sad_plane = sad_plane.with_color(&glm::vec3(0.0, 0.0, 0.0));
                    sad_plane = sad_plane.with_texture(&sad_texture, 2);
                }

                new_obj1 = new_obj1
                    .with_color(&glm::vec3(0.0, 0.0, 0.0))
                    .with_texture(&texture_pool.as_slice()[rand_int3], 1);
                new_obj2 = new_obj2
                    .with_color(&glm::vec3(0.0, 0.0, 0.0))
                    .with_texture(&texture_pool.as_slice()[rand_int4], rand_int7);

                new_obj3 = new_obj3
                    .with_color(&glm::vec3(0.0, 0.0, 0.0))
                    .with_texture(&texture_pool.as_slice()[rand_int5], rand_int8);
            }

            if game_state.score == 10 {
                println!("Texturas!");
            }

            if game_state.score >= 12 {
                current_shader = &phong_illumination;
                plane = plane
                    .with_specular_reflectance(&glm::vec3(0.6, 0.2, 0.4))
                    .with_specular_phong_q(&8.0);

                main_obj = main_obj.with_specular_reflectance(&glm::vec3(0.3, 0.3, 0.3));

                new_obj1 = new_obj1.with_specular_reflectance(&gen_random_vec3());
                new_obj2 = new_obj2.with_specular_reflectance(&gen_random_vec3());
                new_obj3 = new_obj3.with_specular_reflectance(&gen_random_vec3());
            }

            if game_state.score == 12 {
                println!("Phong Ilumination!");
            }

            if game_state.score == 14 {
                println!("Camera livre!");
            }

            if game_state.score > 14 {
                move_camera = true;
            }

            if game_state.score > 16 {
                main_obj = main_obj.get_root();
                game_state.current_camera = 1;
            }
            if game_state.score == 16 {
                println!("Primeira Pessoa!")
            }

            game_state.draw_queue.push(new_obj1);

            // Adiciona entre 0 a 2 objetos extras na cena
            if game_state.score > 2 {
                if gen_random_i32() % 3 == 0 {
                    game_state.draw_queue.push(new_obj2);

                    if game_state.score > 5 {
                        if gen_random_i32() % 3 == 0 {
                            game_state.draw_queue.push(new_obj3);
                        }
                    }
                }
            }

            game_state.should_add_obj = false;
        }

        // Atualiza estado da camera look-at
        if move_camera {
            // Camera movel
            look_at_camera.pos = glm::vec4(
                main_obj.get_matrix().matrix.c3.x,
                main_obj.get_matrix().matrix.c3.y + 0.5,
                main_obj.get_matrix().matrix.c3.z - 0.5,
                look_at_camera.pos.w,
            );

            look_at_camera.front = game_state.look_at;
            look_at_camera.distance = game_state.camera_height - game_state.obj_plane_height;
        } else {
            // Camera fixa
            look_at_camera.pos = glm::vec4(
                look_at_camera.pos.x,
                game_state.camera_height,
                look_at_camera.pos.z,
                look_at_camera.pos.w,
            );

            look_at_camera.front = game_state.look_at;
            look_at_camera.distance = game_state.camera_height - game_state.obj_plane_height;
        }

        look_at_camera.refresh();

        // Atualiza estado da camera free
        free_camera.pos = glm::vec4(
            main_obj.get_matrix().matrix.c3.x,
            main_obj.get_matrix().matrix.c3.y + 0.5,
            main_obj.get_matrix().matrix.c3.z + 0.2,
            free_camera.pos.w,
        );
        free_camera.refresh_as_free_camera();

        if game_state.current_camera == 0 {
            view.update_camera(&look_at_camera);
        } else {
            view.update_camera(&free_camera);
        }

        // Prepara view
        if game_state.is_view_orto {
            view.ortographic().render(&current_shader);
        } else {
            view.perpective().render(&current_shader);
        }

        // Desenha plano
        plane.draw(&current_shader);
        sad_plane.draw(&current_shader);

        // Desenha objetos
        draw_frame(&mut main_obj, &current_shader, &mut game_state);

        // Tempo de renderização de uma frame
        delta_time = timer.elapsed().as_secs_f64();

        // Força framerate
        sleep(Duration::from_secs_f64(glm::max(
            (1.0 / framerate) - delta_time,
            0.0,
        )));

        // Atualiza tempo de renderização após pausa de framerate
        delta_time = timer.elapsed().as_secs_f64();
        gl_window.swap_buffers().unwrap();

        // Interrompe loop
        if game_state.should_break {
            break;
        }
    }
}

pub fn draw_frame(main: &mut SceneObject, shader: &u32, game_state: &mut GameState) {
    main.draw(shader);

    let mut new_items: Vec<SceneObject> = vec![];
    let mut should_add_obj = false;
    let mut score = 0;
    game_state.draw_queue.as_slice().iter().for_each(|item| {
        // Verifica intersecções entre objetos, destroi aqueles que intersectam e pede um objeto novo
        if main.check_intersection(&item) {
            should_add_obj = true;
            score = score + 1;
        } else {
            item.draw(shader);
            new_items.push(item.clone());
        }
    });

    game_state.draw_queue = new_items;
    game_state.should_add_obj = should_add_obj;
    game_state.score = score + game_state.score;
}

#[allow(dead_code)]
pub fn generate_random_obj(base: &SceneObject, obj_plane_height: f32) -> SceneObject {
    let mut new_obj = base.clone();
    let seed1 = gen_random_vec3();
    let seedf1 = gen_random();
    new_obj = new_obj
        .scale(
            glm::max(0.1, glm::min(seedf1 * 0.5, 0.2)),
            glm::max(0.1, glm::min(seedf1 * 0.5, 0.2)),
            glm::max(0.1, glm::min(seedf1 * 0.5, 0.2)),
        )
        .translate(2.5 * seed1.x, obj_plane_height, 2.5 * seed1.z);
    new_obj
}

// Gera numero randomizado entre 0-1
pub fn gen_random() -> f32 {
    (rand::random::<i32>() % 10000) as f32 / 10000.0
}

pub fn gen_random_i32() -> i32 {
    rand::random::<i32>()
}

pub fn gen_random_usize() -> usize {
    rand::random::<usize>()
}

// Gera vec normalizado aletorio
pub fn gen_random_vec4() -> glm::Vec4 {
    normalize_vector(glm::vec4(gen_random(), gen_random(), gen_random(), 0.0))
}

#[allow(dead_code)]
// Gera vec normalizado aletorio
pub fn gen_random_vec3() -> glm::Vec3 {
    let vec4 = gen_random_vec4();
    glm::vec3(vec4.x, vec4.y, vec4.z)
}
