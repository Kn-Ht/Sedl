mod window;
use std::error::Error;

fn run() -> Result<(), Box<dyn Error>> {
    let mut window = window::Window::init()?;

    while !window.should_quit {
        window.handle_events(|event| {
            match event {
                _ => {} // Handle events here
            }
        });

        window.prepare_frame();

        let ui = window.new_imgui_frame();
        /* create imgui UI here */
        ui.show_demo_window(&mut true);

        /* render */
        window.render();
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("ERROR: {e}");
    }
}
