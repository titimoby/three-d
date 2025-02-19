// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args.get(1).map(|a| std::path::PathBuf::from(a))).await;
}

use three_d::*;

pub async fn run(screenshot: Option<std::path::PathBuf>) {
    let window = Window::new(WindowSettings {
        title: "Texture!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl().unwrap();

    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(4.0, 1.5, 4.0),
        vec3(0.0, 1.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        1000.0,
    )
    .unwrap();
    let mut control = OrbitControl::new(*camera.target(), 1.0, 100.0);

    let mut loaded = Loader::load_async(&[
        "examples/assets/skybox_evening/right.jpg",
        "examples/assets/skybox_evening/left.jpg",
        "examples/assets/skybox_evening/top.jpg",
        "examples/assets/skybox_evening/front.jpg",
        "examples/assets/skybox_evening/back.jpg",
        "examples/assets/test_texture.jpg",
        "examples/assets/PenguinBaseMesh.obj",
        "examples/assets/PenguinBaseMesh.mtl",
        "examples/assets/penguin.png",
    ])
    .await
    .unwrap();

    let skybox = Skybox::new(
        &context,
        &loaded
            .cube_image("right", "left", "top", "top", "front", "back")
            .unwrap(),
    )
    .unwrap();
    let mut box_object = Model::new_with_material(
        &context,
        &CpuMesh::cube(),
        ColorMaterial {
            texture: Some(std::rc::Rc::new(
                Texture2D::new(&context, &loaded.image("test_texture").unwrap()).unwrap(),
            )),
            ..Default::default()
        },
    )
    .unwrap();
    box_object.material.render_states.cull = Cull::Back;
    let (penguin_cpu_meshes, penguin_cpu_materials) = loaded.obj("PenguinBaseMesh.obj").unwrap();
    let mut penguin_object = Model::new_with_material(
        &context,
        &penguin_cpu_meshes[0],
        PhysicalMaterial::new(&context, &penguin_cpu_materials[0]).unwrap(),
    )
    .unwrap();
    penguin_object.set_transformation(Mat4::from_translation(vec3(0.0, 1.0, 0.5)));
    penguin_object.material.render_states.cull = Cull::Back;

    let ambient = AmbientLight::new(&context, 0.4, Color::WHITE).unwrap();
    let directional =
        DirectionalLight::new(&context, 2.0, Color::WHITE, &vec3(0.0, -1.0, -1.0)).unwrap();

    // main loop
    window
        .render_loop(move |mut frame_input| {
            let mut redraw = frame_input.first_frame;
            redraw |= camera.set_viewport(frame_input.viewport).unwrap();
            redraw |= control
                .handle_events(&mut camera, &mut frame_input.events)
                .unwrap();

            // draw
            if redraw {
                Screen::write(&context, ClearState::default(), || {
                    render_pass(
                        &camera,
                        &[&box_object, &penguin_object, &skybox],
                        &[&ambient, &directional],
                    )
                })
                .unwrap();
            }

            if let Some(ref screenshot) = screenshot {
                // To automatically generate screenshots of the examples, can safely be ignored.
                FrameOutput {
                    screenshot: Some(screenshot.clone()),
                    exit: true,
                    ..Default::default()
                }
            } else {
                FrameOutput {
                    swap_buffers: redraw,
                    ..Default::default()
                }
            }
        })
        .unwrap();
}
