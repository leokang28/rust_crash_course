extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
        .expect("failed to initialize GTK application");
    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("first gtk app");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("click me");
        button.connect_clicked(|_| {
            println!("clicked");
        });

        window.add(&button);

        window.show_all();
    });

    app.run(&[]);
}
