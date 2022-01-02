use rfraylib::{Color, Draw, Gesture, Image, Key, MouseButton, Rectangle, Size};

const SCREEN_SIZE: Size = Size {
    width: 800,
    height: 450,
};

fn main() -> anyhow::Result<()> {
    let mut system = rfraylib::SystemBuilder::new()
        .window_size(SCREEN_SIZE)
        .window_title("raylib [textures] example - mouse painting")
        .target_fps(120)
        .build()?;

    // Colours to choose from
    let colors = [
        Color::RAYWHITE,
        Color::YELLOW,
        Color::GOLD,
        Color::ORANGE,
        Color::PINK,
        Color::RED,
        Color::MAROON,
        Color::GREEN,
        Color::LIME,
        Color::DARKGREEN,
        Color::SKYBLUE,
        Color::BLUE,
        Color::DARKBLUE,
        Color::PURPLE,
        Color::VIOLET,
        Color::DARKPURPLE,
        Color::BEIGE,
        Color::BROWN,
        Color::DARKBROWN,
        Color::LIGHTGRAY,
        Color::GRAY,
        Color::DARKGRAY,
        Color::BLACK,
    ];

    let colors_recs = (0..colors.len() as i32)
        .map(|i| Rectangle::new(10 + 30 * i + 2 * i, 10, 30, 30))
        .collect::<Vec<_>>();

    let mut color_selected = 0;
    let mut color_selected_prev = color_selected;
    let mut brush_size = 20.0;
    let mut mouse_was_pressed = false;

    let btn_save_rec = Rectangle::new(750, 10, 40, 30);
    let mut show_save_message = false;
    let mut save_message_counter = 0;

    // Create a RenderTexture2D to use as a canvas
    let mut target = rfraylib::RenderTexture::load(SCREEN_SIZE).expect("TODO");

    // Clear render texture before entering the game loop
    system
        .create_texture_canvas(&mut target)
        .clear_background(colors[0]);

    while !system.window().should_close() {
        //
        // Update
        //
        let mouse_pos = system.mouse().get_position();
        let keyboard = system.keyboard();
        let mouse = system.mouse();

        // Move between colors with keys
        if keyboard.is_key_pressed(Key::Right) {
            color_selected = std::cmp::min(colors.len() - 1, color_selected + 1);
        } else if keyboard.is_key_pressed(Key::Left) {
            color_selected = color_selected.saturating_sub(1);
        }

        // Choose color with mouse
        let color_mouse_hover = colors_recs
            .iter()
            .position(|r| mouse_pos.check_collision_point_rec(*r));

        if let Some(i) = color_mouse_hover {
            if mouse.is_button_pressed(MouseButton::Left) {
                color_selected = i;
                color_selected_prev = color_selected;
            }
        }

        // Change brush size
        brush_size = (brush_size + mouse.get_wheel_move() * 5.0)
            .max(2.0)
            .min(50.0);

        if keyboard.is_key_pressed(Key::C) {
            // Clear render texture to clear color
            system
                .create_texture_canvas(&mut target)
                .clear_background(colors[0]);
        }

        if mouse.is_button_down(MouseButton::Left)
            || system.touch().get_gesture_detected() == Gesture::Drag
        {
            // Paint circle into render texture
            // NOTE: To avoid discontinuous circles, we could store
            // previous-next mouse points and just draw a line using brush size
            if mouse_pos.y > 50 {
                system.create_texture_canvas(&mut target).draw_circle(
                    mouse_pos,
                    brush_size,
                    colors[color_selected],
                );
            }
        }
        if mouse.is_button_down(MouseButton::Right) {
            if !mouse_was_pressed {
                color_selected_prev = color_selected;
                color_selected = 0;
            }

            mouse_was_pressed = true;

            // Erase circle from render texture
            if mouse_pos.y > 50 {
                system
                    .create_texture_canvas(&mut target)
                    .draw_circle(mouse_pos, brush_size, colors[0]);
            }
        } else if mouse.is_button_released(MouseButton::Right) && mouse_was_pressed {
            color_selected = color_selected_prev;
            mouse_was_pressed = false;
        }

        // Check mouse hover save button
        let btn_save_mouse_hover = mouse_pos.check_collision_point_rec(btn_save_rec);

        // Image saving logic
        // NOTE: Saving painted texture to a default named image
        if btn_save_mouse_hover && mouse.is_button_released(MouseButton::Left)
            || keyboard.is_key_pressed(Key::S)
        {
            let mut image = Image::load_from_texture(target.texture()).expect("TODO");
            image.flip_vertical();
            image.export("my_amazing_texture_painting.png");
            show_save_message = true;
        }
        if show_save_message {
            // On saving, show a full screen message for 2 seconds
            save_message_counter += 1;
            if save_message_counter > 240 {
                show_save_message = false;
                save_message_counter = 0;
            }
        }

        let is_right_button_down = mouse.is_button_down(MouseButton::Right);

        //
        // Draw
        //
        let mut canvas = system.next_frame();
        canvas.clear_background(Color::RAYWHITE);

        // NOTE: Render texture must be y-flipped due to default OpenGL coordinates (left-bottom)
        canvas.draw_texture_rec(
            target.texture(),
            Rectangle {
                position: (0, 0).into(),
                size: target.texture().size(), //.map(|(w, h)| (w, -h)),
            },
            (0, 0).into(),
            Color::WHITE,
        );

        // Draw drawing circle for reference
        if mouse_pos.y > 50 {
            if is_right_button_down {
                canvas.draw_circle_lines(mouse_pos, brush_size, Color::GRAY);
            } else {
                canvas.draw_circle_lines(mouse_pos, brush_size, colors[color_selected]);
            }
        }

        // Draw top panel
        canvas.draw_rectangle(Rectangle::new(0, 0, SCREEN_SIZE.width, 50), Color::RAYWHITE);
        canvas.draw_line(
            (0, 50).into(),
            (SCREEN_SIZE.width as i32, 50).into(),
            Color::LIGHTGRAY,
        );

        // Draw color selection rectangles.
        for (color, rec) in colors.iter().zip(colors_recs.iter()) {
            canvas.draw_rectangle(*rec, *color);
        }
        canvas.draw_rectangle_lines(Rectangle::new(10, 10, 30, 30), Color::LIGHTGRAY);
        if let Some(i) = color_mouse_hover {
            canvas.draw_rectangle(colors_recs[i], Color::WHITE.fade(0.6));
        }
        canvas.draw_rectangle_lines_ex(
            colors_recs[color_selected]
                .map(|x, y, width, height| (x - 2, y - 2, width + 4, height + 4)),
            2.0,
            Color::BLACK,
        );

        // Draw save image button
        let btn_save_color = if btn_save_mouse_hover {
            Color::RED
        } else {
            Color::BLACK
        };
        canvas.draw_rectangle_lines_ex(btn_save_rec, 2.0, btn_save_color);
        canvas.draw_text("SAVE!", (755, 20).into(), 10, btn_save_color)?;

        // Draw save image message
        if show_save_message {
            canvas.draw_rectangle(
                Rectangle::new(0, 0, SCREEN_SIZE.width, SCREEN_SIZE.height),
                Color::RAYWHITE.fade(0.8),
            );
            canvas.draw_rectangle(Rectangle::new(0, 150, SCREEN_SIZE.width, 80), Color::BLACK);

            canvas.draw_text(
                "IMAGE SAVED:  my_amazing_texture_painting.png",
                (150, 180).into(),
                20,
                Color::RAYWHITE,
            )?;
        }
    }
    Ok(())
}
