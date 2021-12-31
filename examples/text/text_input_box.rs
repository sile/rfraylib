use rfraylib::core::drawing::Draw;
use rfraylib::text::Font;
use rfraylib::Color;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 450.0;
const MAX_INPUT_CHARS: usize = 9;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut system = rfraylib::SystemBuilder::new()
        .window_size((SCREEN_WIDTH, SCREEN_HEIGHT).into())
        .window_title("raylib [text] example - input box")
        .target_fps(10)
        .build()?;

    let text_box = rfraylib::Rectangle {
        x: SCREEN_WIDTH / 2.0 - 100.0,
        y: 180.0,
        width: 225.0,
        height: 50.0,
    };

    let mut frames_counter = 0;
    let mut name = String::new();
    while !system.window().should_close() {
        let mouse_on_text = system
            .mouse()
            .get_position()
            .check_collision_point_rec(text_box);
        if mouse_on_text {
            // Set the window's cursor to the I-Beam
            system
                .mouse_mut()
                .set_cursor(rfraylib::core::input::mouse::MouseCursor::Ibeam);

            // Get char pressed (unicode character) on the queue
            for c in system.keyboard_mut().take_pressed_chars() {
                if c.is_alphanumeric() && name.len() < MAX_INPUT_CHARS {
                    name.push(c);
                }
            }

            if system
                .keyboard()
                .is_key_pressed(rfraylib::core::input::keyboard::Key::Backspace)
            {
                name.pop();
            }
        } else {
            system
                .mouse_mut()
                .set_cursor(rfraylib::core::input::mouse::MouseCursor::Default);
        }

        if mouse_on_text {
            frames_counter += 1;
        } else {
            frames_counter = 0;
        }

        let mut canvas = system.next_frame();
        canvas.clear_background(Color::RAYWHITE);
        canvas.draw_text(
            "PLACE MOUSE OVER INPUT BOX!",
            (240.0, 140.0).into(),
            20,
            Color::GRAY,
        )?;

        canvas.draw_rectangle(text_box, Color::LIGHTGRAY);

        if mouse_on_text {
            canvas.draw_rectangle_lines(text_box, Color::RED);
        } else {
            canvas.draw_rectangle_lines(text_box, Color::DARKGRAY);
        }

        canvas.draw_text(
            &name,
            (text_box.x + 5.0, text_box.y + 8.0).into(),
            40,
            Color::MAROON,
        )?;
        canvas.draw_text(
            &format!("INPUT CHARS: {}/{}", name.len(), MAX_INPUT_CHARS),
            (315.0, 250.0).into(),
            20,
            Color::DARKGRAY,
        )?;

        if mouse_on_text {
            if name.len() < MAX_INPUT_CHARS {
                // Draw blinking underscore char
                if ((frames_counter / 20) % 2) == 0 {
                    canvas.draw_text(
                        "_",
                        (
                            text_box.x + 8.0 + Font::measure_text(&name, 40)?,
                            text_box.y + 12.0,
                        )
                            .into(),
                        40,
                        Color::MAROON,
                    )?;
                }
            } else {
                canvas.draw_text(
                    "Press BACKSPACE to delete chars...",
                    (230.0, 300.0).into(),
                    20,
                    Color::GRAY,
                )?;
            }
        }
    }

    Ok(())
}
