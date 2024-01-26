use imgui::{Context, FontConfig};
use imgui_glow_renderer::glow::HasContext;
use imgui_glow_renderer::AutoRenderer;
use sdl2::video::GLProfile;
use sdl2::{event::Event, keyboard::Keycode, video::Window};

use imgui_sdl2_support::SdlPlatform;
use std::error::Error;

// Create a new glow context.
fn glow_context(window: &Window) -> imgui_glow_renderer::glow::Context {
    unsafe {
        imgui_glow_renderer::glow::Context::from_loader_function(|s| {
            window.subsystem().gl_get_proc_address(s) as _
        })
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;

    let window = video_subsystem
        .window("Hello World!", 500, 500)
        .position_centered()
        .opengl()
        .resizable()
        .allow_highdpi()
        .build()?;

    /* hint SDL to initialize an OpenGL 3.3 core profile context */
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_version(3, 3);
    gl_attr.set_context_profile(GLProfile::Core);

    // opengl
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    let gl = glow_context(&window);

    // imgui
    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(FontConfig {
                oversample_h: 3,
                oversample_v: 3,
                ..Default::default()
            }),
        }]);

    /* create platform and renderer */
    let mut platform = SdlPlatform::init(&mut imgui);
    let mut renderer = AutoRenderer::initialize(gl, &mut imgui).unwrap();

    /* start main loop */
    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            platform.handle_event(&mut imgui, &event);

            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        platform.prepare_frame(&mut imgui, &window, &event_pump);

        let ui = imgui.new_frame();
        /* create imgui UI here */
        ui.show_demo_window(&mut true);

        /* render */
        let draw_data = imgui.render();

        unsafe { renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
        renderer.render(draw_data).unwrap();

        window.gl_swap_window();
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {e}");
    }
}
