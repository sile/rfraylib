fn main() -> anyhow::Result<()> {
    let system = rfraylib::SystemBuilder::new().build()?;
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
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}
