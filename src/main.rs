use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box as GtkBox, Label, Orientation, ScrolledWindow};
use std::fs;

const APP_ID: &str = "org.ekah.BerryPicker";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(ui);
    app.run()
}

fn ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Berry Picker")
        .default_width(200)
        .default_height(300) // Small height now
        .build();

    let vbox = GtkBox::new(Orientation::Vertical, 5);

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            let label = Label::new(Some(&file_name_str));
            label.set_xalign(0.0);
            label.set_margin_top(5);
            label.set_margin_bottom(5);
            label.set_margin_start(10);
            label.set_margin_end(10);
            vbox.append(&label);
        }
    }

    // Create a scrolled window and put the vbox inside
    let scrolled_window = ScrolledWindow::builder()
        .child(&vbox)
        .min_content_height(5 * 50) // Approx 5 labels × ~50px each
        .max_content_height(5 * 50) // Fix max height too
        .build();

    window.set_child(Some(&scrolled_window));
    window.present();
}
