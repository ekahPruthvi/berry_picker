use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Label, Orientation,
    ScrolledWindow, Frame, DragSource, gdk::{ContentProvider,Key}, gio
};
use std::fs;
use std::path::PathBuf;
use gio::File;
use glib::Value;

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
        .default_width(500)
        .default_height(250)
        .resizable(false)
        .build();
       
    let event_controller = gtk::EventControllerKey::new();

    event_controller.connect_key_pressed(|_, key, _, _| {
        match key {
            gtk::gdk::Key::q | gtk::gdk::Key::Escape => {
                std::process::exit(0);
            }
            _ => (),
        }
        glib::Propagation::Proceed
    });
    
    window.add_controller(event_controller);
    
    
    let vbox_main = GtkBox::new(Orientation::Vertical, 5);

    let heading = Label::new(Some("BerryPicker"));
    heading.set_margin_top(10);
    heading.set_margin_bottom(10);
    heading.set_margin_start(10);
    heading.set_margin_end(10);
    heading.set_xalign(0.0);
    heading.set_markup(
        "<span font='Cantarell 20' weight='bold'>BerryPicker</span>"
    );

    vbox_main.append(&heading);

    let vbox_files = GtkBox::new(Orientation::Vertical, 5);

    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries.flatten() {
            let path: PathBuf = entry.path();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            let label = Label::new(Some(&file_name_str));
            label.set_xalign(0.0);
            label.set_margin_top(10);
            label.set_margin_bottom(10);
            label.set_margin_start(10);
            label.set_margin_end(10);

            // Make the label draggable
            make_label_draggable(&label, path);

            vbox_files.append(&label);
        }
    }

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&vbox_files));

    let frame = Frame::new(None);
    frame.set_child(Some(&scrolled_window));
    frame.set_margin_start(10);
    frame.set_margin_end(10);
    frame.set_margin_bottom(10);
    frame.set_margin_top(0);
    frame.set_vexpand(true);

    vbox_main.append(&frame);

    window.set_child(Some(&vbox_main));
    window.present();
}


fn make_label_draggable(label: &Label, path: PathBuf) {
    let drag_source = DragSource::new();
    let drag_source_for_prepare = drag_source.clone();

    label.add_controller(drag_source);

    drag_source_for_prepare.connect_prepare(move |_drag_source, _x, _y| {
        let file = File::for_path(&path);

        // Wrap the file in a value
        let file_value: Value = file.to_value();

        // Create a ContentProvider using the file's value
        let provider = ContentProvider::for_value(&file_value);

        Some(provider)
    });
}
