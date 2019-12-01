use glm::builtin::pow;
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
    pub camera_height: f32,
    pub obj_plane_height: f32,
    pub speed_mult: f64,
    pub look_at: glm::Vec4,
    pub camera_speed_mult: f32,
    pub current_camera: i32,
    pub can_fly: bool,
    pub with_bezier: bool,
    pub curr_x: f64,
    pub dir: f64,
    pub complex_objs: bool,
    pub max_framerate: f64,
    pub progression_multiplier: i32,
    pub lighting_source: glm::Vec4,
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
    let gourad_lambert_illumination = Shader::new(
        "src/data/shader/vertex/goraud_shading_lambert.glsl",
        "src/data/shader/fragment/gourad_fragment_lambert.glsl",
    )
    .program;

    // Compila e linka shaders
    let gourad_phong_illumination = Shader::new(
        "src/data/shader/vertex/goraud_shading_phong.glsl",
        "src/data/shader/fragment/gourad_fragment_phong.glsl",
    )
    .program;

    // Compila e linka shaders
    let lambert_illumination = Shader::new(
        "src/data/shader/vertex/phong_shading.glsl",
        "src/data/shader/fragment/lambert_ilumination.glsl",
    )
    .program;

    let phong_illumination = Shader::new(
        "src/data/shader/vertex/phong_shading.glsl",
        "src/data/shader/fragment/phong_ilumination.glsl",
    )
    .program;
    gl::Enable(gl::DEPTH_TEST);

    let blinn_phong_illumination = Shader::new(
        "src/data/shader/vertex/phong_shading.glsl",
        "src/data/shader/fragment/blinn_phong_ilumination.glsl",
    )
    .program;
    gl::Enable(gl::DEPTH_TEST);

    // Inicializa estado do jogo
    let mut game_state = GameState {
        is_view_orto: true,
        should_break: false,
        should_add_obj: true,
        draw_queue: Vec::new(),
        score: 0,
        camera_height: 0.0,
        obj_plane_height: -20.0,
        speed_mult: 3.0,
        camera_speed_mult: 0.0,
        look_at: glm::vec4(0.0, -1.0, 0.000000000001, 0.0),
        current_camera: 0,
        can_fly: false,
        with_bezier: false,
        curr_x: 0.0,
        dir: 1.0,
        complex_objs: false,
        max_framerate: 90.0,
        progression_multiplier: 1,
        lighting_source: glm::vec4(0.0, 0.0, 0.0, 0.0),
    };

    ////////////////////// Carrega texturas do jogo /////////////////////////

    let (sad_texture, _) = load_texture("src/data/textures/sad.jpg");
    let (pearl_texture, _) = load_texture("src/data/textures/pearl.jpg");

    let (copper_texture, _) = load_texture("src/data/textures/copper.jpg");
    let (gold_texture, _) = load_texture("src/data/textures/gold.jpg");
    let (ice_texture, _) = load_texture("src/data/textures/ice.jpg");
    let (light_wood, _) = load_texture("src/data/textures/light_wood.jpg");

    let (old_wood_texture, _) = load_texture("src/data/textures/old_wood.jpg");
    let (sea_water_texture, _) = load_texture("src/data/textures/sea_water.jpg");
    let (steel_texture, _) = load_texture("src/data/textures/steel.jpg");
    let (earth_texture, _) = load_texture("src/data/textures/earth.jpg");
    let (lava_texture, _) = load_texture("src/data/textures/lava.jpg");
    let (fire_texture, _) = load_texture("src/data/textures/fire.jpg");

    let (glass_texture, _) = load_texture("src/data/textures/glass.jpg");
    let (corn, _) = load_texture("src/data/textures/corn.jpg");

    let (pattern1, _) = load_texture("src/data/textures/pattern1.jpg");
    let (pattern2, _) = load_texture("src/data/textures/pattern2.jpg");

    let texture_pool = vec![
        &pearl_texture,
        &gold_texture,
        &sea_water_texture,
        &copper_texture,
        &steel_texture,
        &ice_texture,
        &light_wood,
        &old_wood_texture,
        &earth_texture,
        &fire_texture,
        &lava_texture,
        &glass_texture,
        &corn,
        &sad_texture,
        &pattern1,
        &pattern2,
    ];
    let plane_pool = vec![&glass_texture, &pattern1];

    /////////////////////// Carrega objs do jogo /////////////////////////////
    let mut plane = SceneObject::new("src/data/objs/plane.obj")
        .scale(5.0, 5.0, 5.0)
        .translate(0.0, game_state.obj_plane_height, 0.0)
        .with_color(&glm::vec3(0.6, 0.6, 0.6));

    let mut sad_plane = SceneObject::new("src/data/objs/plane.obj")
        .scale(1.0, 1.0, 1.0)
        .translate(0.0, game_state.obj_plane_height + 0.001, 0.0)
        .with_color(&glm::vec3(0.6, 0.6, 0.6))
        .with_ambient_reflectance(&glm::vec3(0.0, 0.0, 0.0))
        .with_specular_reflectance(&glm::vec3(0.8, 0.8, 0.8))
        .with_specular_phong_q(&8.0);

    let sad_head =
        SceneObject::new("src/data/objs/sphere.obj").with_color(&glm::vec3(0.0, 0.0, 0.0));

    let mut main_obj = SceneObject::new("src/data/objs/cube.obj")
        .with_color(&glm::vec3(1.0, 1.0, 1.0))
        .scale(0.2, 0.2, 0.2)
        .translate(0.0, game_state.obj_plane_height, -0.0);

    let cow = SceneObject::new("src/data/objs/cow.obj")
        .scale(1.5, 1.5, 1.5)
        .translate(0.0, 0.9, 0.0)
        .with_texture_map_type(1);
    let bunny = SceneObject::new("src/data/objs/bunny.obj")
        .translate(0.0, 0.8, 0.0)
        .with_texture_map_type(1);
    let base_cube = SceneObject::new("src/data/objs/cube.obj").with_texture_map_type(1);

    let sphere = SceneObject::new("src/data/objs/sphere.obj")
        .translate(0.0, 0.4, 0.0)
        .with_color(&glm::vec3(0.6, 0.6, 0.2))
        .with_texture_map_type(3);

    let cylinder = SceneObject::new("src/data/objs/cylinder.obj")
        .translate(0.0, 0.4, 0.0)
        .with_color(&glm::vec3(0.6, 0.6, 0.2))
        .with_texture_map_type(4);

    let pyramid = SceneObject::new("src/data/objs/pyramid.obj")
        .rotate_z(1.5)
        .translate(0.0, 0.4, 0.0)
        .with_color(&glm::vec3(0.6, 0.6, 0.2))
        .with_texture_map_type(4);

    let naked_dude = SceneObject::new("src/data/objs/naked_dude.obj")
        .scale(0.15, 0.15, 0.15)
        .translate(0.0, 0.4, 0.0)
        .with_texture_map_type(3);

    let boat = SceneObject::new("src/data/objs/boat.obj")
        .translate(0.0, 0.6, 0.0)
        .with_texture_map_type(3);

    let house = SceneObject::new("src/data/objs/house.obj")
        .scale(0.25, 0.25, 0.25)
        .translate(0.0, 0.4, 0.0)
        .with_texture_map_type(3);

    // Pool de objs aleatorios
    let complex_obj_pool = vec![&cow, &bunny, &naked_dude, &boat, &house];
    let simple_obj_pool = vec![&base_cube, &sphere, &cylinder, &pyramid];

    let mut current_shader = &default_shader;

    // Inicializa camera livre
    let mut free_camera =
        FreeCamera::new(glm::vec3(0.0, game_state.camera_height, 0.0), &0.0, &0.0);

    // Inicializa camera look at e define vetor fixo
    let mut look_at_camera =
        FreeCamera::new(glm::vec3(0.0, game_state.camera_height, 0.0), &0.0, &0.0);

    look_at_camera.front = game_state.look_at;
    free_camera.front = game_state.look_at;

    let mut last_score = 0;

    // Inicializa camera
    // Inicializa matriz de projeção com a camera criada
    let mut view = View::new(-0.01, -20.0, &look_at_camera);

    // Contador de tempo de frame
    let mut delta_time: f64 = 0.001;

    // Controle de velocidade
    let mut speed = 0.0f64;

    // Define se camera se move junto com obj
    let mut move_camera = false;

    //vetor para calculo de aumento da altura da camera
    let mut delta_vec_x = 0.0;
    let mut delta_vec_y = 0.0;
    let mut delta_vec_z = 0.0;

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
                &mut plane,
                &texture_pool,
            );
        });
        // Gera uma alteração de estado do loop do jogo
        if game_state.should_add_obj {
            println!("{:?}", game_state.score);

            // Aumenta obj apos cada ponto
            // Calcula escalamento original do obj antes de ser escalamento
            let init_x = main_obj.get_matrix().matrix.c0[0];
            let init_y = main_obj.get_matrix().matrix.c1[1];
            let init_z = main_obj.get_matrix().matrix.c2[2];
            // Se pontuação aumentou
            if last_score < game_state.score {
                // Utiliza vetor de translação do obj para realizar uma translação - escalamento - translação
                main_obj = main_obj.tscale(1.0 + 0.0020, 1.0 + 0.0020, 1.0 + 0.0020);
                last_score = game_state.score;
            }
            // Pontuação diminuiu
            else if last_score > game_state.score {
                main_obj = main_obj.tscale(1.0 - 0.0020, 1.0 - 0.0020, 1.0 - 0.0020);
                last_score = game_state.score;
            }

            // Calcula aumento da altura da camera com diferença entre tamanho antes e depois do escalamento
            delta_vec_x = delta_vec_x + main_obj.get_matrix().matrix.c0[0] - init_x;
            delta_vec_y = delta_vec_y + main_obj.get_matrix().matrix.c1[1] - init_y;
            delta_vec_z = delta_vec_z + main_obj.get_matrix().matrix.c2[2] - init_z;

            let mut base_obj0 = &base_cube;
            let mut base_obj1 = &base_cube;
            let mut base_obj2 = &base_cube;
            let mut base_obj3 = &base_cube;
            let mut base_obj4 = &base_cube;

            // Obj base 0 é sempre simples
            let rand_int00 = gen_random_usize() % simple_obj_pool.len();
            base_obj0 = &simple_obj_pool.as_slice()[rand_int00];

            if game_state.complex_objs {
                let rand_int01 = gen_random_usize() % complex_obj_pool.len();
                let rand_int02 = gen_random_usize() % complex_obj_pool.len();
                let rand_int03 = gen_random_usize() % complex_obj_pool.len();
                let rand_int04 = gen_random_usize() % complex_obj_pool.len();

                base_obj1 = &complex_obj_pool.as_slice()[rand_int01];
                base_obj2 = &complex_obj_pool.as_slice()[rand_int02];
                base_obj3 = &complex_obj_pool.as_slice()[rand_int03];
                base_obj4 = &complex_obj_pool.as_slice()[rand_int04];
            } else {
                // Objs aleatorioas
                let rand_int01 = gen_random_usize() % simple_obj_pool.len();
                let rand_int02 = gen_random_usize() % simple_obj_pool.len();
                let rand_int03 = gen_random_usize() % simple_obj_pool.len();
                let rand_int04 = gen_random_usize() % simple_obj_pool.len();

                base_obj1 = &simple_obj_pool.as_slice()[rand_int01];
                base_obj2 = &simple_obj_pool.as_slice()[rand_int02];
                base_obj3 = &simple_obj_pool.as_slice()[rand_int03];
                base_obj4 = &simple_obj_pool.as_slice()[rand_int04];
            }

            // Gera objs aleatorios a serem inseridos na cena
            let mut new_obj0 = generate_random_obj(&base_obj0, game_state.obj_plane_height);
            let mut new_obj1 = generate_random_obj(&base_obj1, game_state.obj_plane_height);
            let mut new_obj2 = generate_random_obj(&base_obj2, game_state.obj_plane_height);
            let mut new_obj3 = generate_random_obj(&base_obj3, game_state.obj_plane_height);
            let mut new_obj4 = generate_random_obj(&base_obj4, game_state.obj_plane_height);
            move_camera = false;

            // Posiveis alterações feitas em iterações anteriores são revertidas
            game_state.look_at = glm::vec4(0.0, -1.0, 0.000000000001, 0.0);
            game_state.camera_height = 0.0;
            game_state.is_view_orto = true;
            current_shader = &default_shader;

            game_state.current_camera = 0;

            // Cor nos objetos novos
            if game_state.score >= 2 * game_state.progression_multiplier {
                new_obj0 = new_obj0.with_color(&gen_random_vec3());
                new_obj1 = new_obj1.with_color(&gen_random_vec3());
                new_obj2 = new_obj2.with_color(&gen_random_vec3());
                new_obj3 = new_obj1.with_color(&gen_random_vec3());
                new_obj4 = new_obj2.with_color(&gen_random_vec3());
            }

            if game_state.score == 2 * game_state.progression_multiplier {
                println!("Cor!")
            }

            // Cor no obj principal
            if game_state.score == 4 * game_state.progression_multiplier {
                main_obj = main_obj.with_color(&gen_random_vec3());
                let rand_plane_color = gen_random_vec3() + glm::vec3(0.2, 0.2, 0.2);
                plane = plane.with_color(&rand_plane_color);
                sad_plane = sad_plane.with_color(&rand_plane_color);
            }
            if game_state.score == 5 * game_state.progression_multiplier {
                look_at_camera.pos.z = 0.0;
                game_state.progression_multiplier = 2;
                game_state.score = game_state.score * 2;
            }

            // Proj perspectiva e camera movel
            if game_state.score >= 6 * game_state.progression_multiplier {
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
            if game_state.score == 6 * game_state.progression_multiplier {
                println!("Proj. Perpesctiva e camera movel!");
            }

            // Ilum de lambert, shading gourdad
            if game_state.score >= 8 * game_state.progression_multiplier {
                current_shader = &gourad_lambert_illumination;
                let rand_int = gen_random_usize() % texture_pool.len();

                // Adiciona esfera no topo do obj
                main_obj = main_obj.add_children(
                    &sad_head
                        .with_texture(&texture_pool.as_slice()[rand_int], 4)
                        .scale(0.35, 0.35, 0.35)
                        .translate(0.5, 1.5, -0.0),
                );
            }
            if game_state.score == 8 * game_state.progression_multiplier {
                println!("Iluminação de lambert, shading gourad!")
            }

            // Texturas
            if game_state.score >= 10 * game_state.progression_multiplier {
                main_obj = main_obj.with_color(&glm::vec3(0.0, 0.0, 0.0));

                // Textutas  aleatorias dos objs a serem criados
                let rand_int_main = gen_random_usize() % texture_pool.len();
                let rand_int0 = gen_random_usize() % texture_pool.len();
                let rand_int1 = gen_random_usize() % texture_pool.len();
                let rand_int2 = gen_random_usize() % texture_pool.len();
                let rand_int3 = gen_random_usize() % texture_pool.len();
                let rand_int4 = gen_random_usize() % texture_pool.len();

                // Textura do plano

                let rand_intp = gen_random_usize() % plane_pool.len();

                // Tipos de mapeamentos de textura aleatorios
                let rand_int_type0 = (gen_random_i32() % 5) + 1;
                let rand_int_type1 = (gen_random_i32() % 5) + 1;

                if main_obj.get_texture_override() == 0 {
                    main_obj = main_obj.with_texture(&texture_pool.as_slice()[rand_int_main], 1);
                }

                if plane.get_texture_override() == 0 {
                    plane = plane.with_color(&glm::vec3(0.0, 0.0, 0.0));
                    plane = plane.with_texture(&plane_pool.as_slice()[rand_intp], 2);
                    sad_plane = sad_plane.with_color(&glm::vec3(0.0, 0.0, 0.0));
                    sad_plane = sad_plane.with_texture(&sad_texture, 2);
                }

                // Objeto mais comum mapeado com textura padrão
                new_obj0 = new_obj0.with_color(&glm::vec3(0.0, 0.0, 0.0)).with_texture(
                    &texture_pool.as_slice()[rand_int0],
                    new_obj1.get_texture_map_type(),
                );

                // Segundo objeto mais comum mapeado com mapeamento linear
                new_obj1 = new_obj1
                    .with_color(&glm::vec3(0.0, 0.0, 0.0))
                    .with_texture(&texture_pool.as_slice()[rand_int1], 1);

                // Outros objs mapeados com textura aleatoria
                new_obj2 = new_obj2
                    .with_color(&glm::vec3(0.0, 0.0, 0.0))
                    .with_texture(&texture_pool.as_slice()[rand_int2], rand_int_type0);
                new_obj3 = new_obj3
                    .with_color(&glm::vec3(0.0, 0.0, 0.0))
                    .with_texture(&texture_pool.as_slice()[rand_int3], rand_int_type1);

                // Objeto menos comum mapeado com um ponto da textura ou textura padrao do obj
                new_obj4 = new_obj4
                    .with_color(&glm::vec3(0.0, 0.0, 0.0))
                    .with_texture(&texture_pool.as_slice()[rand_int4], 0);
            }

            if game_state.score == 10 * game_state.progression_multiplier {
                println!("Texturas!");
            }

            // Desenha objs complexos
            if game_state.score >= 12 * game_state.progression_multiplier {
                game_state.complex_objs = true;
            }
            if game_state.score == 12 * game_state.progression_multiplier {
                println!("Objs complexos!");
            }

            // Camera junto com obj principal
            if game_state.score >= 14 * game_state.progression_multiplier {
                move_camera = true;
            }

            // Iluminação de lambert, phong shading
            if game_state.score >= 16 * game_state.progression_multiplier {
                current_shader = &lambert_illumination;
            }
            if game_state.score == 16 * game_state.progression_multiplier {
                println!("Iluminação de lambert, phong shading!")
            }

            // Troca para camera livre em primeira pessoa
            if game_state.score >= 18 * game_state.progression_multiplier {
                main_obj = main_obj.get_root();
                game_state.current_camera = 1;
            }
            if game_state.score == 18 * game_state.progression_multiplier {
                println!("Primeira Pessoa!")
            }

            // Iluminação de phong, gourad shading
            if game_state.score >= 20 * game_state.progression_multiplier {
                current_shader = &gourad_phong_illumination;
            }
            if game_state.score == 20 * game_state.progression_multiplier {
                println!("Iluminação de phong, gourad shading!")
            }

            // Ilum de phong, phong shading
            if game_state.score >= 22 * game_state.progression_multiplier {
                current_shader = &phong_illumination;
                plane = plane;

                let rand3 = gen_random();
                let rand4 = gen_random();

                new_obj1 = new_obj1.with_specular_reflectance(&glm::vec3(1.0, 1.0, 1.0));

                // especular e ambiente aleatorio e q de phong
                new_obj2 = new_obj2
                    .with_specular_reflectance(&glm::vec3(rand3, rand3, rand3))
                    .with_ambient_reflectance(&glm::vec3(rand4, rand4, rand4))
                    .with_specular_phong_q(&glm::pow(2.0, (&gen_random_i32() % 8) as f32));

                // Objeto com refletancia ambiente fixa
                new_obj3 = new_obj3
                    .with_color(&glm::vec3(1.0, 1.0, 1.0))
                    .with_specular_reflectance(&glm::vec3(1.0, 1.0, 1.0));

                // Objeto reflexivo
                new_obj4 = new_obj4
                    .with_color(&glm::vec3(1.0, 1.0, 1.0))
                    .with_ambient_reflectance(&glm::vec3(1.0, 1.0, 1.0))
                    .with_specular_reflectance(&glm::vec3(1.0, 1.0, 1.0));
            }

            if game_state.score == 22 * game_state.progression_multiplier {
                println!("Iluminação de phong, phong shading");
            }

            if game_state.score >= 24 * game_state.progression_multiplier {
                println!(" Iluminação relativa a fonte de luz!");
                if game_state.lighting_source == glm::vec4(0.0, 0.0, 0.0, 0.0) {
                    game_state.lighting_source = glm::vec4(0.0, -18.0, 0.0, 1.0);
                }
            };
            if game_state.score == 24 * game_state.progression_multiplier {
                println!(" Iluminação relativa a fonte de luz!");
            }
            if game_state.score >= 28 * game_state.progression_multiplier {
                new_obj2 = new_obj2.with_specular_phong_q(&16.0);
                current_shader = &blinn_phong_illumination;
            };

            if game_state.score == 28 * game_state.progression_multiplier {
                println!(" Iluminação de blinn phong !");
            }

            // Adiciona um obj novo na fila de desenho
            game_state.draw_queue.push(new_obj0);

            // Adiciona entre 0 a 4 objetos extras na cena
            if game_state.score > 2 * game_state.progression_multiplier {
                if gen_random_i32() % 5 < 2 {
                    game_state.draw_queue.push(new_obj1);
                }
            }

            if game_state.score > 5 * game_state.progression_multiplier {
                if gen_random_i32() % 5 == 0 {
                    game_state.draw_queue.push(new_obj2);
                }
            }

            if game_state.score > 7 * game_state.progression_multiplier {
                if gen_random_i32() % 5 == 0 {
                    game_state.draw_queue.push(new_obj3);
                }
            }

            if game_state.score > 9 * game_state.progression_multiplier {
                if gen_random_i32() % 5 == 0 {
                    game_state.draw_queue.push(new_obj4);
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

        // Atualiza camera sem calcular vetor de direção
        look_at_camera.refresh();

        // Atualiza estado da camera livre, corrigindo possiveis aumentos de altura da camera
        free_camera.pos = glm::vec4(
            main_obj.get_matrix().matrix.c3.x + delta_vec_x,
            main_obj.get_matrix().matrix.c3.y + 0.5 + delta_vec_y,
            main_obj.get_matrix().matrix.c3.z + 0.2,
            free_camera.pos.w,
        );
        // Recarrega camera utilizando angulos para calcular vetor de direção da camera
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
            (1.0 / game_state.max_framerate) - delta_time,
            0.0,
        )));

        // Atualiza tempo de renderização após pausa de framerate
        delta_time = timer.elapsed().as_secs_f64();
        gl_window.swap_buffers().unwrap();

        // Atualiza variavel acumuladora de [0..1] para curvas de bezier utilizando variação do tempo
        if game_state.curr_x > 1.0 {
            game_state.dir = -1.0;
        }

        if game_state.curr_x <= 0.0 {
            game_state.dir = 1.0;
        }
        game_state.curr_x =
            game_state.curr_x + (timer.elapsed().as_secs_f64() * game_state.dir) / 10.0;

        // Interrompe loop
        if game_state.should_break {
            break;
        }
    }
}

#[allow(dead_code, unused_assignments)]
pub fn draw_frame(main: &mut SceneObject, shader: &u32, game_state: &mut GameState) {
    // Gerencia colisões, movimento e desenha frame
    main.draw(shader);

    let mut new_items: Vec<SceneObject> = vec![];
    let mut should_add_obj = false;
    let mut score = 0;

    // Desenha cada objeto da fila de desenho e checa interseções
    game_state.draw_queue.as_slice().iter().for_each(|item| {
        // Verifica intersecções entre objetos, destroi aqueles que intersectam e pede um objeto novo
        let mut is_intersecting = false;

        let b03 = pow(1.0 - game_state.curr_x, 3.0) as f32;
        let b23 = 3.0 * pow(game_state.curr_x, 2.0) as f32 * (1.0 - game_state.curr_x) as f32;
        let b13 = 3.0 * game_state.curr_x as f32 * pow(1.10 - game_state.curr_x, 2.0) as f32;
        let b33 = pow(game_state.curr_x, 3.0) as f32;

        let p1 = glm::vec4(-2.5, 0.4, 0.0, 0.0);
        let p2 = glm::vec4(-2.00, 1.8, 1.25, 0.0);
        let p3 = glm::vec4(2.0, 1.8, 0.5, 0.0);
        let p4 = glm::vec4(4.5, 0.0, 1.25, 0.0);
        let curve = (p1 * b03 + p2 * b13 + p3 * b23 + p4 * b33) / 4.0;

        if game_state.with_bezier {
            is_intersecting = main.check_intersection(&item.translate(curve.x, curve.y, curve.z))
        } else {
            is_intersecting = main.check_intersection(&item)
        }

        if is_intersecting {
            // Remove obj da cena e indica criação de novo obj
            should_add_obj = true;
            score = score + 1;
        } else {
            // Desenha alguns objs com mov em curva de bezier
            if game_state.with_bezier {
                item.with_lighting_source_override(&game_state.lighting_source)
                    .trot_y(2.0 * 3.14 * game_state.curr_x as f32)
                    .trot_x(2.0 * 3.14 * game_state.curr_x as f32)
                    .translate(curve.x, curve.y, curve.z)
                    .draw(shader);

                new_items.push(item.clone());
            } else {
                item.with_lighting_source_override(&game_state.lighting_source)
                    .draw(shader);
                new_items.push(item.clone());
            }
        }
    });

    // Atualiza estado do jogo
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
        .translate(4.6 * seed1.x, obj_plane_height, 4.6 * seed1.z);
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
