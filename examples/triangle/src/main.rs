// Entry point for non-wasm
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = std::env::args().collect();
    run(args.get(1).map(|a| std::path::PathBuf::from(a)));
}

use three_d::*;

pub fn run(screenshot: Option<std::path::PathBuf>) {
    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Triangle!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context = window.gl().unwrap();

    // Create a camera
    let mut camera = Camera::new_perspective(
        &context,
        window.viewport().unwrap(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    )
    .unwrap();

    // Create a CPU-side mesh consisting of a single colored triangle
    let positions = vec![
        vec3(0.5, -0.5, 0.0),  // bottom right
        vec3(-0.5, -0.5, 0.0), // bottom left
        vec3(0.0, 0.5, 0.0),   // top
    ];
    let colors = vec![
        Color::new(255, 0, 0, 255), // bottom right
        Color::new(0, 255, 0, 255), // bottom left
        Color::new(0, 0, 255, 255), // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };

    // Construct a model, with a default color material, thereby transferring the mesh data to the GPU
    let mut model = Model::new(&context, &cpu_mesh).unwrap();

    // Start the main render loop
    window.render_loop(move |frame_input: FrameInput| // Begin a new frame with an updated frame input
    {
        // Ensure the viewport matches the current window viewport which changes if the window is resized
        camera.set_viewport(frame_input.viewport).unwrap();

        // Start writing to the screen and clears the color and depth
        Screen::write(&context, ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0), || {
            // Set the current transformation of the triangle
            model.set_transformation(Mat4::from_angle_y(radians((frame_input.accumulated_time * 0.005) as f32)));

            // Render the triangle with the color material which uses the per vertex colors defined at construction
            model.render(&camera, &[])?;
            Ok(())
        }).unwrap();

        if let Some(ref screenshot) = screenshot {
            // To automatically generate screenshots of the examples, can safely be ignored.
            FrameOutput {
                screenshot: Some(screenshot.clone()),
                exit: true,
                ..Default::default()
            }
        } else {
            // Returns default frame output to end the frame
            FrameOutput::default()
        }
    }).unwrap();
}
