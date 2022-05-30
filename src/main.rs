use gtk::{
    glib::clone,
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{ButtonExt, GtkWindowExt, WidgetExt},
    Application, ApplicationWindow, Builder, Button, Label,
};
use std::sync::{Arc, Mutex};

fn main() {
    let application = Application::new(None, Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let main_window_src = include_str!("../target/ui/main_window.ui");
    let builder = Builder::from_string(main_window_src);

    let main_window: ApplicationWindow = builder.object("main_window").unwrap();
    let counter_label: Label = builder.object("counter_label").unwrap();
    let increment_button: Button = builder.object("increment_button").unwrap();
    let decrement_button: Button = builder.object("decrement_button").unwrap();

    let counter = Arc::new(Mutex::new(0));

    increment_button.connect_clicked(clone!(@strong counter_label, @strong counter => move |_| {
        let mut counter = counter.lock().unwrap();
        *counter += 1;

        counter_label.set_label(&counter.to_string());
    }));
    decrement_button.connect_clicked(clone!(@strong counter_label, @strong counter => move |_| {
        let mut counter = counter.lock().unwrap();
        *counter -= 1;

        counter_label.set_label(&counter.to_string());
    }));

    counter_label.set_text(&counter.lock().unwrap().to_string());

    main_window.set_application(Some(application));

    main_window.show();
}
