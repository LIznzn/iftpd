use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("My Rusty Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(300, 300);

    let button = gtk::Button::with_label("Click me!");
    window.add(&button);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.my.rs"), Default::default());
    application.connect_activate(build_ui);
    application.run();
    // println!("Hello, world!");
}
