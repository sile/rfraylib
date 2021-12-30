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
    println!("Clipboard: {:?}", system.window().get_clipboard_text());
    for _ in 0..10 {
        println!(
            "Cursor: hidden={}, on_screen={}",
            system.cursor().is_hidden(),
            system.cursor().is_on_screen()
        );
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    Ok(())
}
