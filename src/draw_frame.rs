use handle_input::handle_input;
use models::draw::Draw;
use models::scene_object::SceneObject;
use shader::shader_program::Shader;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use world::camera::Camera;
use world::view::View;

// Estrutura representando combinação final de objeto e shader a ser desenhado
#[allow(dead_code)]
pub struct Drawable<'a> {
    object: &'a SceneObject,
    shader: &'a u32,
}

#[allow(dead_code)]
impl<'a> Drawable<'a> {
    fn new(object: &'a SceneObject, shader: &'a u32) -> Self {
        Drawable { object, shader }
    }

    fn update_object(&self, object: &'a SceneObject) -> Self {
        Drawable { object, ..*self }
    }

    fn update_shader(&self, shader: &'a u32) -> Self {
        Drawable { shader, ..*self }
    }
}

#[allow(dead_code)]
pub unsafe fn game_loop(
    events_loop: &mut glutin::EventsLoop,
    gl_window: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::Window>,
) {
    // Compila e linka shaders
    let program = Shader::new(
        "src/data/shader/vertex.glsl",
        "src/data/shader/fragment.glsl",
    )
    .program;

    gl::Enable(gl::DEPTH_TEST);

    let cow = SceneObject::new("src/data/objs/cow.obj");

    let framerate = 120.0;

    // Inicializa camera
    let mut camera = Camera::new(0.0, 0.0, 2.5);

    // Inicializa matrizes de view e projeção com a camera criada
    let mut view = View::new(-0.01, -10.0, &camera);

    let mut delta_time: f64 = 0.001;
    let mut is_view_orto = false;
    let mut should_break = false;

    loop {
        // Inicializa cronometro de tempo de renderização de uma frame
        let timer = Instant::now();
        let speed = delta_time * 1.0;
        let mut draw_queue: Vec<Drawable> = Vec::new();

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);

        // Trata eventos
        events_loop.poll_events(|event| {
            handle_input(
                event,
                &mut should_break,
                &mut is_view_orto,
                &mut camera,
                &speed,
            );
        });

        // Atualiza possiveis modificações de camera;
        view.update_camera(&camera);

        // Prepara view
        if is_view_orto {
            view.ortographic().render(&program);
        } else {
            view.render(&program);
        }

        // Prepara objetos para serem desenhados
        draw_queue.push(Drawable::new(&cow, &program));
        draw_frame(&draw_queue);

        // Tempo de renderização de uma frame
        delta_time = timer.elapsed().as_secs_f64();
        sleep(Duration::from_secs_f64(glm::max(
            (1.0 / framerate) - delta_time,
            0.0,
        )));

        delta_time = timer.elapsed().as_secs_f64();
        gl_window.swap_buffers().unwrap();

        if should_break {
            break;
        }
    }
}

pub fn draw_frame(draw_list: &Vec<Drawable>) {
    draw_list.as_slice().iter().for_each(|item| {
        item.object.draw(item.shader);
    });
}
