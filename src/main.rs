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
    use gtk::{SearchEntry, Widget};

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

    // Use Overlay to have background + UI
    let overlay = gtk::Overlay::new();

    // Create and set the SVG background
    let background_picture = gtk::Picture::new();
    background_picture.set_can_target(false); // Non-interactive
    background_picture.set_content_fit(gtk::ContentFit::Cover);
    background_picture.set_filename(Some("background.svg"));
    overlay.set_child(Some(&background_picture)); // Background at bottom

    // Main vertical layout
    let vbox_main = GtkBox::new(Orientation::Vertical, 5);

    let heading = Label::new(Some("BerryPicker"));
    heading.set_margin_top(10);
    heading.set_margin_bottom(10);
    heading.set_margin_start(10);
    heading.set_margin_end(10);
    heading.set_xalign(0.0);
    heading.set_markup("<span font='Cantarell 20' weight='bold'>BerryPicker</span>");
    vbox_main.append(&heading);

    // Search entry
    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search files..."));
    search_entry.set_margin_start(10);
    search_entry.set_margin_end(10);
    vbox_main.append(&search_entry);

    // VBox to hold file labels
    let vbox_files = GtkBox::new(Orientation::Vertical, 5);

    // Populate with file labels
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

            make_label_draggable(&label, path);

            vbox_files.append(&label);
        }
    }

    // Scrollable area for file labels
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

    // Add vbox_main to overlay (on top of background)
    overlay.add_overlay(&vbox_main);

    window.set_child(Some(&overlay));
    window.present();

    // --- Search filtering logic ---
    let vbox_files_clone = vbox_files.clone();
    search_entry.connect_search_changed(move |entry| {
        let query = entry.text().to_lowercase();

        let mut child = vbox_files_clone.first_child();
        while let Some(widget) = child {
            if let Some(label) = widget.downcast_ref::<Label>() {
                let label_text = label.text().to_string().to_lowercase();
                if label_text.contains(&query) {
                    widget.set_visible(true);
                } else {
                    widget.set_visible(false);
                }
            }
            child = widget.next_sibling();
        }
    });
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
