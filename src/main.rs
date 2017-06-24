#[macro_use]
extern crate conrod;

use conrod::{widget, Colorable, Positionable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};

fn main() {
    let ans = iter(identity);
    println!("Final: {:?}", ans);
}

fn identity(text: &str) -> String {
    println!("Processing: {}", text);
    String::from(text)
}

pub fn iter<F>(process: F) -> Option<String>
    where F: Fn(&str) -> String
{
    // Build the window.
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_fullscreen(glium::glutin::get_primary_monitor())
        .with_title("Hello Conrod!")
        .with_multisampling(4)
        .build_glium()
        .unwrap();

    // construct our `Ui`.
    let dimensions = glium::glutin::get_primary_monitor().get_dimensions();
    let mut ui = conrod::UiBuilder::new([dimensions.0 as f64, dimensions.1 as f64]).build();

    // Generate the widget identifiers.
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    const FONT_PATH: &'static str = "/usr/share/fonts/TTF/Ubuntu-M.ttf";
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut input_text = &mut String::from("Hello world");

    // Poll events from the window.
    let mut last_update = std::time::Instant::now();
    let mut ui_needs_update = true;
    'main: loop {

        // We don't want to loop any faster than 60 FPS, so wait until it has been at least
        // 16ms since the last yield.
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events: Vec<_> = display.poll_events().collect();

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !ui_needs_update {
            events.extend(display.wait_events().next());
        }

        // Reset the needs_update flag and time this update.
        ui_needs_update = false;
        last_update = std::time::Instant::now();

        // Handle all events.
        for event in events {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
                ui_needs_update = true;
            }

            match event {
                // Break from the loop upon `Escape`.
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                    glium::glutin::Event::Closed => {
                        break 'main;
                    }
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed,
                                                    _,
                                                    Some(keycode)) => {
                    println!("Key: {:?}", keycode);
                }
                _ => {}
            }
        }

        // Instantiate all widgets in the GUI.
        {
            let ui = &mut ui.set_widgets();
            if let Some(answer) = set_widgets(ui, &ids, input_text, &process) {
                return Some(answer);
            }
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }

    None
}

widget_ids!(struct Ids { canvas, scrollbar, input, output });

fn set_widgets<F>(ui: &mut conrod::UiCell,
                  ids: &Ids,
                  input_text: &mut String,
                  process: &F)
                  -> Option<String>
    where F: Fn(&str) -> String
{
    widget::Canvas::new()
        .scroll_kids_vertically()
        .color(conrod::color::DARK_CHARCOAL)
        .set(ids.canvas, ui);

    for edit in widget::TextEdit::new(input_text)
            .color(conrod::color::WHITE)
            .mid_top_of(ids.canvas)
            .center_justify()
            .set(ids.input, ui) {
        *input_text = edit;
        // If contains \n, return this (without \n) as answer
        if input_text.contains("\n") {
            return Some(input_text.replace("\n", ""));
        }
    }

    widget::Text::new(&process(&input_text))
        .color(conrod::color::LIGHT_RED)
        .middle_of(ids.canvas)
        .center_justify()
        .set(ids.output, ui);

    widget::Scrollbar::y_axis(ids.canvas)
        .auto_hide(true)
        .set(ids.scrollbar, ui);

    None
}
