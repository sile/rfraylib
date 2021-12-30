use rfraylib::Draw;

fn main() -> anyhow::Result<()> {
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
        let mut canvas = system.next_frame();
        canvas.clear_background(rfraylib::Color::GOLD);
    }
    Ok(())
}
