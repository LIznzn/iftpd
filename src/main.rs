use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("iftpd-ui.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("MainWindow").expect("Couldn't get MainWindow");
    window.set_application(Some(application));

    window.show_all();
}