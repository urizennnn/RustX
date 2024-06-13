use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Box, Orientation, Switch};

const APP_ID: &str = "org.gtk_rs.GObjectProperties1";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create the switch
    let switch = Switch::new();
    let switch2 = Switch::new();
    // Set and then immediately obtain active property
    switch.set_active(true);
    switch2.set_active(true);
    switch.connect_activate(|switch| {
        switch.set_active(false);
        let switch_active = switch.is_active();
        println!("The active property of switch is {}", switch_active);
    });
    let switch_active = switch.is_active();
    switch
        .bind_property("active", &switch2, "active")
        .transform_to(|_, active: bool| {
            print!("The active property of switch is {}", active);
            Some(!active)
        })
        .transform_to(|_, active: bool| Some(active))
        .sync_create()
        .build();

    // This prints: "The active property of switch is true"
    println!("The active property of switch is {}", switch_active);

    // Set up box
    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&switch);
    gtk_box.append(&switch2);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
