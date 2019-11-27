use drawable::Drawable;
use handle_input::handle_input;
use models::draw::Draw;
use models::matrix::GLMatrix;
use models::matrix::MatrixTransform;
use models::obj_model::ObjModel;
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

    let cow = SceneObject::new("src/data/objs/cow.obj");
    let cube = SceneObject::new("src/data/objs/cube.obj")
        .with_color(&glm::vec3(1.0, 1.0, 1.0))
        .scale(0.2, 0.2, 0.2)
        .translate(0.0, -10.0, -0.0);

    let cube2 = SceneObject::new("src/data/objs/cube.obj")
        .with_color(&glm::vec3(0.0, 1.0, 1.0))
        .scale(2.2, 0.2, 0.2)
        .translate(-1.0, -10.0, -0.0);

    let framerate = 120.0;

    // Inicializa camera livre
    let mut camera = FreeCamera::new(glm::vec3(0.0, 0.0, 0.0), &0.0, &0.0);

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

    loop {
        // Inicializa cronometro de tempo de renderização de uma frame
        let timer = Instant::now();

        // 1.0 unidade por segundo
        speed = delta_time * 1.0;

        // Lista de objetos a serem desenhados
        let mut draw_queue: Vec<Drawable> = Vec::new();

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

        // Verifica intersecções entre objetos
        check_intersection(&cube, &cube2);

        // Prepara objetos para serem desenhados
        draw_queue.push(Drawable::new(&cube, &program));
        draw_queue.push(Drawable::new(&cube2, &program));

        draw_frame(&draw_queue);

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

pub fn check_intersection(obj1: &SceneObject, obj2: &SceneObject) -> bool {
    //let model_translation = obj1.model.matrix.c3;

    // Utiliza transação do obj para calcular pos global
    let obj1_t = obj1.get_matrix().matrix.c3;
    let obj2_t = obj2.get_matrix().matrix.c3;

    let obj1_bbox_min = obj1.get_bbox_min();
    let obj1_bbox_max = obj1.get_bbox_max();

    let obj2_bbox_min = obj2.get_bbox_min();
    let obj2_bbox_max = obj2.get_bbox_max();

    // Pos global da bbox  do obj1
    let obj1_bbox_min_pos = obj1.get_matrix().matrix
        * glm::vec4(obj1_bbox_min.x, obj1_bbox_min.y, obj1_bbox_min.z, 0.0)
        + obj1_t;

    let obj1_bbox_max_pos = obj1.get_matrix().matrix
        * glm::vec4(obj1_bbox_max.x, obj1_bbox_max.y, obj1_bbox_max.z, 0.0)
        + obj1_t;

    // Pos global da bbox  do obj2
    let obj2_bbox_min_pos = obj1.get_matrix().matrix
        * glm::vec4(obj2_bbox_min.x, obj2_bbox_min.y, obj2_bbox_min.z, 0.0)
        + obj2_t;

    let obj2_bbox_max_pos = obj1.get_matrix().matrix
        * glm::vec4(obj2_bbox_max.x, obj2_bbox_max.y, obj2_bbox_max.z, 0.0)
        + obj2_t;

    let result = check_bbox_bbox_intersection(
        &obj1_bbox_min_pos,
        &obj1_bbox_max_pos,
        &obj2_bbox_min_pos,
        &obj2_bbox_max_pos,
    );
    println!("{:?}", result);
    println!(
        "{:?} {:?} {:?} {:?}",
        obj1_bbox_min_pos, obj1_bbox_max_pos, obj2_bbox_min_pos, obj2_bbox_max_pos,
    );

    false
}

#[allow(dead_code)]
pub fn check_point_bbox_intersection(
    bbox_min: &glm::Vec4,
    bbox_max: &glm::Vec4,
    point: &glm::Vec4,
) -> bool {
    return (point.x >= bbox_min.x && point.x <= bbox_max.x)
        && (point.y >= bbox_min.y && point.y <= bbox_max.y)
        && (point.z >= bbox_min.z && point.z <= bbox_max.z);
}

#[allow(dead_code)]
pub fn check_bbox_bbox_intersection(
    bbox1_min: &glm::Vec4,
    bbox1_max: &glm::Vec4,
    bbox2_min: &glm::Vec4,
    bbox2_max: &glm::Vec4,
) -> bool {
    return (bbox1_min.x <= bbox2_max.x && bbox1_max.x >= bbox2_min.x)
        && (bbox1_min.y <= bbox2_max.y && bbox1_max.y >= bbox2_min.y)
        && (bbox1_min.z <= bbox2_max.z && bbox1_max.z >= bbox2_min.z);
}

// Desenha items com seus respectivos shaders
pub fn draw_frame(draw_list: &Vec<Drawable>) {
    draw_list.as_slice().iter().for_each(|item| {
        item.object.draw(item.shader);
    });
}
