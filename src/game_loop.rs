use handle_input::handle_input;
use models::draw::Draw;
use models::matrix::MatrixTransform;
use models::scene_object::SceneObject;
use shader::shader_program::Shader;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use world::free_camera::FreeCamera;
use world::view::View;

#[allow(dead_code, unused_assignments)]
pub unsafe fn game_loop(
    events_loop: &mut glutin::EventsLoop,
    gl_window: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>,
) {
    // Compila e linka shaders
    let program = Shader::new(
        "src/data/shader/vertex/default.glsl",
        "src/data/shader/fragment/default.glsl",
    )
    .program;

    gl::Enable(gl::DEPTH_TEST);

    let mut cube = SceneObject::new("src/data/objs/cube.obj")
        .with_color(&glm::vec3(1.0, 1.0, 1.0))
        .scale(0.2, 0.2, 0.2)
        .translate(0.0, -10.0, -0.0);

    let cube2 = SceneObject::new("src/data/objs/cube.obj")
        .with_color(&glm::vec3(0.0, 1.0, 1.0))
        .scale(0.2, 0.2, 0.2)
        .translate(0.1, -10.0, -0.0);

    let framerate = 120.0;

    // Inicializa camera livre
    //let camera = FreeCamera::new(glm::vec3(0.0, 0.0, 0.0), &0.0, &0.0);

    // Inicializa camera look at e define vetor fixo
    let mut look_at_camera = FreeCamera::new(glm::vec3(0.0, 0.0, 0.0), &0.0, &0.0);
    look_at_camera.front = glm::vec4(0.0, -1.0, 0.000000000001, 0.0);

    // Inicializa camera
    // // Inicializa matrizes de view e projeção com a camera criada
    let mut view = View::new(-0.01, -20.0, &look_at_camera);

    // Contador de tempo de frame
    let mut delta_time: f64 = 0.001;

    // Controles de estado de loop
    let mut is_view_orto = false;
    let mut should_break = false;
    let mut speed = 0.0f64;
    let mut should_add_obj = true;

    let mut rand_pos_x: f32 = (rand::random::<i32>() % 1000) as f32 / 1000.0;
    let mut rand_pos_z: f32 = (rand::random::<i32>() % 1000) as f32 / 1000.0;

    // Lista de objetos a serem desenhados
    let mut draw_queue: Vec<SceneObject> = Vec::new();

    loop {
        // Inicializa cronometro de tempo de renderização de uma frame
        let timer = Instant::now();

        // 1.0 unidade por segundo
        speed = delta_time * 1.0;

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);

        // Trata eventos
        events_loop.poll_events(|event| {
            handle_input(
                event,
                &mut should_break,
                &mut is_view_orto,
                &mut look_at_camera,
                &mut view,
                &mut speed,
                &mut cube,
            );
        });

        look_at_camera.refresh();
        view.update_camera(&look_at_camera);

        // Prepara view
        if is_view_orto {
            view.ortographic().render(&program);
        } else {
            view.render(&program);
        }

        if should_add_obj {
            rand_pos_x = (rand::random::<i32>() % 1000) as f32 / 1000.0;
            rand_pos_z = (rand::random::<i32>() % 1000) as f32 / 1000.0;
            draw_queue.push(cube2.clone().translate(rand_pos_x, 0.0, rand_pos_z));
            should_add_obj = false;
        }
        draw_frame(&cube, &program, &mut draw_queue, &mut should_add_obj);

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
        if should_break {
            break;
        }
    }
}

// Desenha items com seus respectivos shaders
pub fn draw_frame(
    main: &SceneObject,
    shader: &u32,
    draw_list: &mut Vec<SceneObject>,
    should_add_obj: &mut bool,
) {
    main.draw(shader);

    let mut new_items: Vec<SceneObject> = vec![];

    draw_list.as_slice().iter().for_each(|item| {
        // Verifica intersecções entre objetos, destroi aqueles que intersectam e pede um objeto novo
        if main.check_intersection(&item) {
            *should_add_obj = true;
        } else {
            item.draw(shader);
            new_items.push(item.clone());
        }
    });

    *draw_list = new_items;
}
