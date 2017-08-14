use std;

use conrod::{self, widget, Colorable, Positionable, Widget, Sizeable, Borderable};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::Surface;

use conrod::backend::glium::glium::glutin::VirtualKeyCode;

use config::Config;
use gui_result::GuiResult;

struct State {
    input_text: String,
    selected: usize,
}

#[doc = "Runs the menu with the default config.
 The process function will filter the output based on the input."]
pub fn run<F, T>(process: F) -> GuiResult<T>
where
    F: Fn(&str) -> Vec<T>,
    T: Into<String> + From<String> + Clone,
{
    run_config(process, &Config::default())
}

#[doc = "Runs with a configuration. See [run](fn.run.html)"]
pub fn run_config<F, T>(process: F, configuration: &Config) -> GuiResult<T>
where
    F: Fn(&str) -> Vec<T>,
    T: Into<String> + From<String> + Clone,
{
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_fullscreen(glium::glutin::get_primary_monitor())
        .with_title("Rmenu");
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // construct our `Ui`.
    let dimensions = glium::glutin::get_primary_monitor().get_dimensions();
    let mut ui = conrod::UiBuilder::new([dimensions.0 as f64, dimensions.1 as f64]).build();

    // Generate the widget identifiers.
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    ui.fonts.insert_from_file(&configuration.font).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut state = State {
        input_text: String::new(),
        selected: 0,
    };

    let mut answer = GuiResult::Cancel;

    events_loop.run_forever(|event| {

        // Break from the loop upon `Escape` or closed window.
        match event.clone() {
            glium::glutin::Event::WindowEvent { event, .. } => {
                match event {
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape), ..
                        },
                        ..
                    } => return glium::glutin::ControlFlow::Break,
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(keycode),
                            state: glium::glutin::ElementState::Pressed,
                            ..
                        },
                        ..
                    } => {
                        match keycode {
                            VirtualKeyCode::Up => {
                                state.selected = state.selected.saturating_sub(1);
                            }
                            VirtualKeyCode::Down => {
                                state.selected = state.selected.saturating_add(1);
                            }
                            _ => {}
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }

        // Use the `winit` backend feature to convert the winit event to a conrod one.
        let input = match conrod::backend::winit::convert_event(event, &display) {
            None => return glium::glutin::ControlFlow::Continue,
            Some(input) => input,
        };

        // Handle the input with the `Ui`.
        ui.handle_event(input);

        // Set the widgets.
        {
           let ui = &mut ui.set_widgets();
           if let Some(ans) = set_widgets(ui, &ids, &mut state, &process, configuration) {
               answer = ans;
               return glium::glutin::ControlFlow::Break;
           }
        }

        // Draw the `Ui` if it has changed.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }

        glium::glutin::ControlFlow::Continue
    });

    // TODO: Try to close the window

    answer
}

widget_ids!(struct Ids { canvas, scrollbar, input, output });

fn set_widgets<F, T>(
    ui: &mut conrod::UiCell,
    ids: &Ids,
    state: &mut State,
    process: &F,
    config: &Config,
) -> Option<GuiResult<T>>
where
    F: Fn(&str) -> Vec<T>,
    T: Into<String> + From<String> + Clone,
{
    let canvas = widget::Canvas::new().scroll_kids_vertically().color(
        config.canvas_color,
    );
    canvas.set(ids.canvas, ui);
    let height = canvas.get_h(ui).unwrap();

    let list = process(&state.input_text);
    state.selected = std::cmp::min(state.selected, list.len().saturating_sub(1));

    let mut is_confirmed = false;

    for event in widget::TextBox::new(&state.input_text)
        .color(config.input_color)
        .xy(
            [
                0.0,
                height / 2.0 - config.input_top_padding - config.input_size[1] / 2.0,
            ],
        )
        .wh(config.input_size)
        .center_justify()
        .border(config.input_border)
        .border_color(config.input_border_color)
        .set(ids.input, ui)
    {
        match event {
            widget::text_box::Event::Update(edit) => {
                state.input_text = edit;
            }
            widget::text_box::Event::Enter => {
                if list.is_empty() {
                    return Some(GuiResult::Custom(T::from(state.input_text.clone())));
                } else {
                    is_confirmed = true;
                }
            }
        }
    }

    let (mut items, _) = widget::List::flow_down(list.len())
        .down_from(ids.input, config.input_size[1] + config.output_top_padding)
        .wh(config.output_size)
        .set(ids.output, ui);
    while let Some(item) = items.next(ui) {
        let i = item.i;
        let contents: String = list[i].clone().into();
        let text = widget::Text::new(&contents)
            .color(if i == state.selected {
                if is_confirmed {
                    return Some(GuiResult::Option(list[i].clone()));
                }
                config.selected_color
            } else {
                config.unselected_color
            })
            .center_justify();
        item.set(text, ui);
    }

    None
}
