use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, MenuItem};

fn main() {
    let application = gtk::Application::new(
        Some("com.starmoe.dev.iftpd.application"),
        Default::default(),
    );
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("iftpd-ui.glade");
    let builder = Builder::from_string(glade_src);

    let window_main: ApplicationWindow = builder.object("window_main").expect("Couldn't get window_main");
    window_main.set_application(Some(application));

    let menu_about: MenuItem = builder.object("menu_about").expect("Couldn't get menu_about");
    menu_about.connect_activate(|_| {
        println!("触发关于弹窗！")
    });

    window_main.show_all();
}