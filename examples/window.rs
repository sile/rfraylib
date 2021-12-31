use rfraylib::Draw;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut system = rfraylib::SystemBuilder::new().build()?;
    for monitor in system.monitors().iter() {
        println!(
            "[{}] name={:?}, refresh-rate={}, size(pixel)={:?}, size(mm)={:?}",
            monitor.number().get(),
            monitor.get_name(),
            monitor.get_refresh_rate(),
            monitor.get_size(),
            monitor.get_physical_size(),
        );
    }

    if let Some(sound) = rfraylib::audio::Sound::load("/tmp/coin.wav") {
        println!("Start sound");
        system.audio_device_mut().play_sound(&sound);
        while system.audio_device().is_sound_playing(&sound) {}
    }

    while !system.window().should_close() {
        {
            let mut canvas = system.next_frame();
            canvas.clear_background(rfraylib::Color::GOLD);
            for x in 10..20 {
                for y in 10..20 {
                    canvas.draw_pixel((x as f32, y as f32).into(), rfraylib::Color::RED);
                }
            }
            canvas.draw_fps((100.0, 100.0).into());
        }
        if system.window().is_file_dropped() {
            println!("File dropped!");
        }
        for c in system.keyboard_mut().take_pressed_chars() {
            println!("Pressed Char: {}", c);
        }
        for k in system.keyboard_mut().take_pressed_keys() {
            println!("Pressed Key: {:?}", k);
        }
        if let Some(b) = system.get_gamepad_button_pressed() {
            println!("Gamepad: {:?}", b);
        }
        if system.mouse().get_delta() != (0.0, 0.0).into() {
            println!("Mouse: {:?}", system.mouse().get_position());
        }
        for point in system.touch().get_touch_points() {
            println!("Touch: {:?}", point);
        }
    }
    Ok(())
}
