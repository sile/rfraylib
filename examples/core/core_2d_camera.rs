use rand::Rng;
use rfraylib::{Camera, Color, Draw, Key, Rectangle};

const MAX_BUILDINGS: usize = 100;
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 450;

fn main() -> anyhow::Result<()> {
    let mut system = rfraylib::SystemBuilder::new()
        .window_size((SCREEN_WIDTH, SCREEN_HEIGHT).into())
        .window_title("raylib [core] example - 2d camera")
        .target_fps(60)
        .build()?;

    let mut player = Rectangle::new(400, 280, 40, 40);
    let mut buildings = Vec::new();
    let mut build_colors = Vec::new();
    let mut spacing = 0;
    let mut rng = rand::thread_rng();
    for _ in 0..MAX_BUILDINGS {
        let width: u32 = rng.gen_range(50..200);
        let height: u32 = rng.gen_range(100..800);
        buildings.push(Rectangle::new(
            -6000 + spacing,
            SCREEN_HEIGHT as i32 - 130 - height as i32,
            width,
            height,
        ));
        spacing += width as i32;

        build_colors.push(Color::rgb(
            rng.gen_range(200..240),
            rng.gen_range(200..240),
            rng.gen_range(200..250),
        ));
    }

    let mut camera = Camera {
        target: player.position.map(|x, y| (x + 20, y + 20)),
        offset: (SCREEN_WIDTH as i32 / 2, SCREEN_HEIGHT as i32 / 2).into(),
        rotation: 0.0,
        zoom: 1.0,
    };

    while !system.window().should_close() {
        // Player movement
        if system.keyboard().is_key_down(Key::Right) {
            player.position.x += 2;
        } else if system.keyboard().is_key_down(Key::Left) {
            player.position.x -= 2;
        }

        // Camera target follows player
        camera.target = player.position.map(|x, y| (x + 20, y + 20));

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
            canvas.draw_rectangle(Rectangle::new(-6000, 320, 13000, 8000), Color::DARKGRAY);

            for (b, c) in buildings.iter().copied().zip(build_colors.iter().copied()) {
                canvas.draw_rectangle(b, c);
            }

            canvas.draw_rectangle(player, Color::RED);

            canvas.draw_line(
                (camera.target.x, -(SCREEN_HEIGHT as i32) * 10).into(),
                (camera.target.x, SCREEN_HEIGHT as i32 * 10).into(),
                Color::GREEN,
            );
            canvas.draw_line(
                (-(SCREEN_WIDTH as i32) * 10, camera.target.y).into(),
                (SCREEN_WIDTH as i32 * 10, camera.target.y).into(),
                Color::GREEN,
            );
        }

        canvas.draw_text("SCREEN AREA", (640, 10).into(), 20, Color::RED)?;

        canvas.draw_rectangle(Rectangle::new(0, 0, SCREEN_WIDTH, 5), Color::RED);
        canvas.draw_rectangle(Rectangle::new(0, 5, 5, SCREEN_HEIGHT - 10), Color::RED);
        canvas.draw_rectangle(
            Rectangle::new(SCREEN_WIDTH as i32 - 5, 5, 5, SCREEN_HEIGHT - 10),
            Color::RED,
        );
        canvas.draw_rectangle(
            Rectangle::new(0, SCREEN_HEIGHT as i32 - 5, SCREEN_WIDTH, 5),
            Color::RED,
        );
        canvas.draw_rectangle(Rectangle::new(10, 10, 250, 113), Color::SKYBLUE.fade(0.5));
        canvas.draw_rectangle_lines(Rectangle::new(10, 10, 250, 113), Color::BLUE);

        canvas.draw_text(
            "Free 2d camera controls:",
            (20, 20).into(),
            10,
            Color::BLACK,
        )?;
        canvas.draw_text(
            "- Right/Left to move Offset",
            (40, 40).into(),
            10,
            Color::DARKGRAY,
        )?;
        canvas.draw_text(
            "- Mouse Wheel to Zoom in-out",
            (40, 60).into(),
            10,
            Color::DARKGRAY,
        )?;
        canvas.draw_text("- A / S to Rotate", (40, 80).into(), 10, Color::DARKGRAY)?;
        canvas.draw_text(
            "- R to reset Zoom and Rotation",
            (40, 100).into(),
            10,
            Color::DARKGRAY,
        )?;
    }

    Ok(())
}
