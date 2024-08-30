use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};
use rand::random;

fn main() {
    let app = Application::builder()
        .application_id("com.marcosdanielr.ui-demo")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let label = Label::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let button = Button::builder()
        .label("Hello")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&label);
    content.append(&button);

    let window = ApplicationWindow::builder()
        .title("UI Demo")
        .application(app)
        .child(&content)
        .build();

    button.connect_clicked(move |_| hello(&label));

    window.show();
}

fn hello(label: &Label) {
    if random() {
        label.set_text("Hello!");
    } else {
        label.set_text("Hi!");
    }
}
