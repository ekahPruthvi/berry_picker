use gtk::prelude::*;
use gtk::{
    glib, Application, ApplicationWindow, Box as GtkBox, Label, Orientation,
    ScrolledWindow, Frame, DragSource, gdk::ContentProvider, gio
};
use std::fs;
use std::path::PathBuf;
use gio::File;
use glib::Value;

const APP_ID: &str = "org.ekah.BerryPicker";

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_OPEN) // <-- ADD THIS LINE
        .build();

    app.connect_activate(|app| {
        ui(app, vec![]); // no files
    });

    app.connect_open(|app, files, _| {
        let file_paths = files.iter()
            .filter_map(|f| f.path()) // get PathBuf
            .collect::<Vec<_>>();

        ui(app, file_paths); // pass opened files
    });

    app.run()
}

fn ui(app: &Application, files: Vec<PathBuf>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Berry Picker")
        .default_width(500)
        .default_height(100)
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
        "<span font='Cantarell 18' weight='bold'>BerryPicker</span>"
    );

    vbox_main.append(&heading);

    let vbox_files = GtkBox::new(Orientation::Vertical, 5);
    let mut _count= 0;
    if files.is_empty() {
        // No files passed, list current dir
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                _count+=1;
                let label = Label::new(Some(&file_name_str));
                label.set_xalign(0.0);
                label.set_margin_top(10);
                label.set_margin_bottom(10);
                label.set_margin_start(10);
                label.set_margin_end(10);

                make_label_draggable(&label, path);

                vbox_files.append(&label);
            }
        }
    } else {
        // Files were passed
        for path in files {
            if path.exists() {
                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("<unknown>");
                
                let label = Label::new(Some(file_name));
                _count+=1;
                label.set_xalign(0.0);
                label.set_margin_top(10);
                label.set_margin_bottom(10);
                label.set_margin_start(10);
                label.set_margin_end(10);

                make_label_draggable(&label, path);

                vbox_files.append(&label);
            }
        }
    }

    let scrolled_window = ScrolledWindow::new();
    scrolled_window.set_child(Some(&vbox_files));

    let frame = Frame::new(None);
    frame.set_child(Some(&scrolled_window));
    frame.set_margin_start(10);
    frame.set_margin_end(10);
    frame.set_margin_bottom(1);
    frame.set_margin_top(0);
    frame.set_vexpand(true);

    vbox_main.append(&frame);

    let no_of_entities = Label::new(None);
    no_of_entities.set_margin_top(1);
    no_of_entities.set_margin_bottom(8);
    no_of_entities.set_margin_start(10);
    no_of_entities.set_margin_end(10);
    no_of_entities.set_xalign(1.0);
    no_of_entities.set_markup(&format!(
        "<span font='Cantarell 8' weight='bold' >Berry Count: {}</span>",
        _count
    ));

    vbox_main.append(&no_of_entities);

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
