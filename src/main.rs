// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let app = AppWindow::new()?;
    
    // Run Slint UI
    // Handle the menu_clicked callback
    app.on_menu_clicked(|page_name| {
        println!("Menu clicked: {}", page_name);
    });
    app.run()?;

    Ok(())
}
