use gtk::{prelude::*};
use crate::layout::build_layout;

pub fn build_application(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Leash");
    window.set_icon_name(Some("cpu"));
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(500, 70);

    let layout_opt = build_layout();
    if let Some(layout) = layout_opt {
        window.add(&layout);
    }
    else {
        let label = gtk::Label::new(Some("Failed to get CPU info"));
        window.add(&label);
    }

    window.show_all();
}