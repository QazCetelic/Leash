mod cpu_info;
mod cpu_restrict;
mod freq_util;
mod layout;
mod application;

use gtk::{prelude::*};
use crate::application::build_application;

fn main() {
    let application = gtk::Application::new(Some("org.qaz.Leash"), Default::default());
    application.connect_activate(build_application);
    application.run();
}