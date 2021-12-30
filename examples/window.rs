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
    while !system.window().should_close() {
        {
            let mut canvas = system.next_frame();
            canvas.clear_background(rfraylib::Color::GOLD);
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
    }
    Ok(())
}
