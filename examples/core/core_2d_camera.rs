use rand::Rng;
use rfraylib::{Camera, Color, Draw, Key, Rectangle};

const MAX_BUILDINGS: usize = 100;
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 450.0;

fn main() -> anyhow::Result<()> {
    let mut system = rfraylib::SystemBuilder::new()
        .window_size((SCREEN_WIDTH, SCREEN_HEIGHT).into())
        .window_title("raylib [core] example - 2d camera")
        .target_fps(60)
        .build()?;

    let mut player = Rectangle::new(400.0, 280.0, 40.0, 40.0);
    let mut buildings = Vec::new();
    let mut build_colors = Vec::new();
    let mut spacing = 0.0;
    let mut rng = rand::thread_rng();
    for _ in 0..MAX_BUILDINGS {
        let width = rng.gen_range(50.0..200.0);
        let height = rng.gen_range(100.0..800.0);
        buildings.push(Rectangle {
            width,
            height,
            y: SCREEN_HEIGHT - 130.0 - height,
            x: -6000.0 + spacing,
        });
        spacing += width;

        build_colors.push(Color::rgb(
            rng.gen_range(200..240),
            rng.gen_range(200..240),
            rng.gen_range(200..250),
        ));
    }

    let mut camera = Camera {
        target: (player.x + 20.0, player.y + 20.0).into(),
        offset: (SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0).into(),
        rotation: 0.0,
        zoom: 1.0,
    };

    while !system.window().should_close() {
        // Player movement
        if system.keyboard().is_key_down(Key::Right) {
            player.x += 2.0;
        } else if system.keyboard().is_key_down(Key::Left) {
            player.x -= 2.0;
        }

        // Camera target follows player
        camera.target = (player.x + 20.0, player.y + 20.0).into();

        // Camera rotation controls
        if system.keyboard().is_key_down(Key::A) {
            camera.rotation -= 1.0;
        } else if system.keyboard().is_key_down(Key::S) {
            camera.rotation += 1.0;
        }

        // Limit camera rotation to 80 degrees (-40 to 40)
        if camera.rotation > 40.0 {
            camera.rotation = 40.0;
        } else if camera.rotation < -40.0 {
            camera.rotation = -40.0;
        }

        // Camera zoom controls
        camera.zoom += system.mouse().get_wheel_move() * 0.05;

        if camera.zoom > 3.0 {
            camera.zoom = 3.0;
        } else if camera.zoom < 0.1 {
            camera.zoom = 0.1;
        }

        // Camera reset (zoom and rotation)
        if system.keyboard().is_key_pressed(Key::R) {
            camera.zoom = 1.0;
            camera.rotation = 0.0;
        }

        let mut canvas = system.next_frame();
        canvas.clear_background(Color::RAYWHITE);
        {
            let mut canvas = canvas.with_camera(camera);
            canvas.draw_rectangle(
                Rectangle::new(-6000.0, 320.0, 13000.0, 8000.0),
                Color::DARKGRAY,
            );

            for (b, c) in buildings.iter().copied().zip(build_colors.iter().copied()) {
                canvas.draw_rectangle(b, c);
            }

            canvas.draw_rectangle(player, Color::RED);

            canvas.draw_line(
                (camera.target.x, -SCREEN_HEIGHT * 10.0).into(),
                (camera.target.x, SCREEN_HEIGHT * 10.0).into(),
                Color::GREEN,
            );
            canvas.draw_line(
                (-SCREEN_WIDTH * 10.0, camera.target.y).into(),
                (SCREEN_WIDTH * 10.0, camera.target.y).into(),
                Color::GREEN,
            );
        }

        canvas.draw_text("SCREEN AREA", (640.0, 10.0).into(), 20, Color::RED)?;

        canvas.draw_rectangle(Rectangle::new(0.0, 0.0, SCREEN_WIDTH, 5.0), Color::RED);
        canvas.draw_rectangle(
            Rectangle::new(0.0, 5.0, 5.0, SCREEN_HEIGHT - 10.0),
            Color::RED,
        );
        canvas.draw_rectangle(
            Rectangle::new(SCREEN_WIDTH - 5.0, 5.0, 5.0, SCREEN_HEIGHT - 10.0),
            Color::RED,
        );
        canvas.draw_rectangle(
            Rectangle::new(0.0, SCREEN_HEIGHT - 5.0, SCREEN_WIDTH, 5.0),
            Color::RED,
        );
        canvas.draw_rectangle(
            Rectangle::new(10.0, 10.0, 250.0, 113.0),
            Color::SKYBLUE.fade(0.5),
        );
        canvas.draw_rectangle_lines(Rectangle::new(10.0, 10.0, 250.0, 113.0), Color::BLUE);

        canvas.draw_text(
            "Free 2d camera controls:",
            (20.0, 20.0).into(),
            10,
            Color::BLACK,
        )?;
        canvas.draw_text(
            "- Right/Left to move Offset",
            (40.0, 40.0).into(),
            10,
            Color::DARKGRAY,
        )?;
        canvas.draw_text(
            "- Mouse Wheel to Zoom in-out",
            (40.0, 60.0).into(),
            10,
            Color::DARKGRAY,
        )?;
        canvas.draw_text(
            "- A / S to Rotate",
            (40.0, 80.0).into(),
            10,
            Color::DARKGRAY,
        )?;
        canvas.draw_text(
            "- R to reset Zoom and Rotation",
            (40.0, 100.0).into(),
            10,
            Color::DARKGRAY,
        )?;
    }

    Ok(())
}
