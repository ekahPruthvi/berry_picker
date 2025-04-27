use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Label, Orientation,
    ScrolledWindow, Frame,
};
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
        .default_height(300)
        .resizable(false)
        .build();

    // Main vertical layout
    let vbox_main = GtkBox::new(Orientation::Vertical, 5);

    // Heading label
    let heading = Label::new(Some("BerryPicker"));
    heading.set_margin_top(10);
    heading.set_margin_bottom(10);
    heading.set_margin_start(10);
    heading.set_margin_end(10);
    heading.set_xalign(0.5); // Center align heading text
    vbox_main.append(&heading);

    // VBox for file list
    let vbox_files = GtkBox::new(Orientation::Vertical, 5);

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();
            let label = Label::new(Some(&file_name_str));
            label.set_xalign(0.0);
            label.set_margin_top(10);
            label.set_margin_bottom(10);
            label.set_margin_start(10);
            label.set_margin_end(10);
            vbox_files.append(&label);
        }
    }

    // Make vbox_files scrollable
    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&vbox_files));

    // Frame around the scrolled window
    let frame = Frame::new(None);
    frame.set_child(Some(&scrolled_window));
    frame.set_margin_start(10);
    frame.set_margin_end(10);
    frame.set_margin_bottom(10);
    frame.set_margin_top(0);

    // Make the frame expand vertically to fill the empty space
    frame.set_vexpand(true);  // This makes the frame fill up the vertical space

    // Add frame (with scrolled list) to main vbox
    vbox_main.append(&frame);

    window.set_child(Some(&vbox_main));
    window.present();
}
