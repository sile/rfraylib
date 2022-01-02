use rfraylib::core::drawing::Draw;
use rfraylib::text::Font;
use rfraylib::Color;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 450;
const MAX_INPUT_CHARS: usize = 9;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut system = rfraylib::SystemBuilder::new()
        .window_size((SCREEN_WIDTH, SCREEN_HEIGHT).into())
        .window_title("raylib [text] example - input box")
        .target_fps(10)
        .build()?;

    let text_box = rfraylib::Rectangle {
        position: (SCREEN_WIDTH as i32 / 2 - 100, 180).into(),
        size: (225, 50).into(),
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
            (240, 140).into(),
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
            text_box.position.map(|x, y| (x + 5, y + 8)).into(),
            40,
            Color::MAROON,
        )?;
        canvas.draw_text(
            &format!("INPUT CHARS: {}/{}", name.len(), MAX_INPUT_CHARS),
            (315, 250).into(),
            20,
            Color::DARKGRAY,
        )?;

        if mouse_on_text {
            if name.len() < MAX_INPUT_CHARS {
                // Draw blinking underscore char
                let font_size = Font::measure_text(&name, 40)? as i32;
                if ((frames_counter / 20) % 2) == 0 {
                    canvas.draw_text(
                        "_",
                        text_box
                            .position
                            .map(|x, y| (x + 8 + font_size, y + 12))
                            .into(),
                        40,
                        Color::MAROON,
                    )?;
                }
            } else {
                canvas.draw_text(
                    "Press BACKSPACE to delete chars...",
                    (230, 300).into(),
                    20,
                    Color::GRAY,
                )?;
            }
        }
    }

    Ok(())
}
