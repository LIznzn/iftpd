use gtk::prelude::*;
use gtk::{CheckButton, Stack, StackTransitionType};

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("iftpd(Test Mode)");
        window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600 , 500);


    let box1 = gtk::Box::new(gtk::Orientation::Vertical, 10);

    window.add(&box1);

    let stack1 = gtk::Stack::new();
    stack1.set_transition_type(StackTransitionType::SlideLeftRight);
    stack1.set_transition_duration(100);


    let page1 = gtk::Grid::new();
    stack1.add_titled(&page1, "server", "服務器");


    let label11 = gtk::Label::new(None);
    label11.set_markup("<big> page1 </big>");
    page1.add(&label11);


    let label1 = gtk::Label::new(None);
    label1.set_markup("<big>A fancy label</big>");
    stack1.add_titled(&label1, "log", "運行日誌");

    let label2 = gtk::Label::new(None);
    label2.set_markup("<big>A fancy label2</big>");
    stack1.add_titled(&label2, "user_management", "用戶管理");


    let label3 = gtk::Label::new(None);
    label3.set_markup("<big>A fancy label3</big>");
    stack1.add_titled(&label3, "misc", "其他選項");



    let stack_switcher = gtk::StackSwitcher::new();
    stack_switcher.set_stack(Some(&stack1));
    


    box1.pack_start(&stack_switcher,false,true, 0);
    box1.pack_start(&stack1,true,true,0);



    let label1 = gtk::Label::new(None);

    // checkbutton = Gtk.CheckButton(label="Click me!")
    // stack.add_titled(checkbutton, "check", "Check Button")
    // 
    // label = Gtk.Label()
    // label.set_markup("<big>A fancy label</big>")
    // stack.add_titled(label, "label", "A label")
    // 
    // stack_switcher = Gtk.StackSwitcher()
    // stack_switcher.set_stack(stack)
    // vbox.pack_start(stack_switcher, True, True, 0)
    // vbox.pack_start(stack, True, True, 0)


    // let label1:Label = gtk::Label::new(Some("Test label"));
    // window.add(&label1);



    // let button = gtk::Button::with_label("Click me!");
    // window.add(&button);




    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.github.my.rs"), Default::default());
    application.connect_activate(build_ui);
    application.run();
    // println!("Hello, world!");
}
