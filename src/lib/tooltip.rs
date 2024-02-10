use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

pub fn generate_tooltip(x: i32, y: i32, definition: String) {
    let application = Application::builder().build();
    application.connect_activate(move |app| {
        let label = Label::new(Some(&definition));
        let window = ApplicationWindow::builder()
        .application(app)
        .child(&label)
        .build();
        window.set_decorated(false);
        window.move_(x, y);
        window.show_all();
        window.connect_focus_out_event(|window,_| {
            println!("Focused out.....");
            window.close();
            gtk::glib::Propagation::Stop
        });
        window.connect_key_press_event(|window, key| {
            if key.keyval() == gdk::keys::constants::Escape {
                window.close();
            }
            gtk::glib::Propagation::Stop
        });
    });
    application.run();
}