mod cpu_info;
mod restrict;
mod freq_util;
mod layout;
mod application;
mod layout_ram;
mod ram_usage;
mod layout_cpu;

use gtk::{prelude::*};
use crate::application::build_application;

fn main() {
    let application = gtk::Application::new(Some("org.qaz.Leash"), Default::default());
    application.connect_activate(build_application);
    application.run();
}