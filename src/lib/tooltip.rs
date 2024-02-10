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
        window.move_(x, y);
        window.show_all();
    });
    application.run();
}