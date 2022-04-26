use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Orientation, TextView};

pub fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    let tv = TextView::builder()
        .height_request(50)
        .width_request(200)
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let b = Box::builder().orientation(Orientation::Vertical).build();

    b.append(&tv);

    // Create a window
    let window = ApplicationWindow::builder()
        .title("Ausblick - Simple .msg Viewer")
        .application(app)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .child(&b)
        .build();

    // Present window
    window.present();
}
