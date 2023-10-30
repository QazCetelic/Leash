use std::time::Duration;
use gtk::{Frame, glib};
use gtk::glib::{clone, Continue};
use gtk::prelude::{ContainerExt, FrameExt, ProgressBarExt, SwitchExt, WidgetExt};
use crate::{ram_usage, restrict};

pub(crate) fn layout_ram() -> Option<Frame> {
    let frame = gtk::Frame::new(Some("RAM"));
    frame.set_label_xalign(0.5);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    frame.add(&vbox);
    vbox.set_margin(10);

    let ram_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 15);
    vbox.add(&ram_hbox);
    ram_hbox.set_halign(gtk::Align::Center);
    ram_hbox.set_margin_top(10);
    ram_hbox.set_tooltip_text(Some("Changes overcommit variables to prevent overcommitting RAM"));

    let ram_label = gtk::Label::new(Some("Prevent overcommitting RAM"));
    ram_hbox.add(&ram_label);

    let switch = gtk::Switch::new();
    switch.connect_active_notify(clone!(@weak switch => move |_| {
        let restrict = switch.is_active();
        let is_restricted = restrict::ram_is_restricted();
        if restrict != is_restricted {
            restrict::ram_restrict(restrict);
        }
    }));
    let update_switch_state = glib::clone!(@weak switch => move || {
        let restricted = restrict::ram_is_restricted();
        switch.set_active(restricted);
    });
    update_switch_state();
    glib::timeout_add_local(Duration::from_millis(2500), move || {
        update_switch_state();
        Continue(true)
    });
    ram_hbox.add(&switch);

    let ram_usage_bar = gtk::ProgressBar::new();
    vbox.add(&ram_usage_bar);
    ram_usage_bar.set_show_text(true);

    let update_ram_info = glib::clone!(@weak ram_usage_bar => move || {
        let mem_info_opt = ram_usage::mem_info();
        if let Some(mem_info) = mem_info_opt {
            let total_ram_gb = (*mem_info.get("MemTotal").expect("Failed to get total RAM") as f64) / 1024.0 / 1024.0;
            let free_ram_gb = (*mem_info.get("MemFree").expect("Failed to get free RAM") as f64) / 1024.0 / 1024.0;
            ram_usage_bar.set_fraction((total_ram_gb - free_ram_gb) / total_ram_gb);
            ram_usage_bar.set_text(Some(&format!("RAM usage: {:.2} GiB / {:.2} GiB", total_ram_gb - free_ram_gb, total_ram_gb)));
        }
        else {
            ram_usage_bar.set_text(Some("Failed to fetch RAM usage"));
        }
    });
    update_ram_info();
    glib::timeout_add_local(Duration::from_millis(250), move || {
        update_ram_info();
        Continue(true)
    });

    Some(frame)
}
