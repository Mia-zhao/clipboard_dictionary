use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label};

pub fn generate_tooltip(x: i32, y: i32, definition: String) {
    let application = Application::builder().build();
    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .build();
        let label = Label::builder()
            .label(&definition)
            .margin(10)
            .max_width_chars(80)
            .wrap(true)
            .build();

        window.set_child(Some(&label));
        window.set_decorated(false);
        window.set_keep_above(true);
        window.move_(x, y);
        window.show_all();
        window.connect_focus_out_event(|window,_| {
            println!("Focused out.....");
            window.close();
            gtk::glib::Propagation::Proceed
        });
        window.connect_key_press_event(|window, key| {
            if key.keyval() == gdk::keys::constants::Escape {
                window.close();
            }
            gtk::glib::Propagation::Proceed
        });
    });
    application.run();
}