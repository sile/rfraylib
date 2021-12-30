fn main() -> anyhow::Result<()> {
    let system = rfraylib::SystemBuilder::new().build()?;
    for monitor in system.window().get_monitors() {
        println!(
            "[{}] name={:?}, refresh-rate={}, size(pixel)={}x{}, size(mm)={}x{}",
            monitor.index(),
            monitor.get_name(),
            monitor.get_refresh_rate(),
            monitor.get_width(),
            monitor.get_height(),
            monitor.get_physical_width(),
            monitor.get_physical_height()
        );
    }
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}
