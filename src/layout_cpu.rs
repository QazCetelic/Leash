use std::cmp::min;
use std::time::Duration;
use gtk::{prelude::*, glib};
use gtk::glib::clone;
use crate::cpu_info::{core_count, current_freq_avg_mhz};
use crate::cpu_info::current_freq_max_mhz;
use crate::cpu_info::freq_min_mhz;
use crate::cpu_info::freq_max_mhz;
use crate::cpu_info::temperature;
use crate::restrict::set_max_freq_ghz;
use crate::freq_util::{mhz_to_ghz, round_to_100mhz};

pub(crate) fn layout_cpu(layout: &gtk::Box) -> Option<()> {
    let frame = gtk::Frame::new(Some("CPU"));
    layout.add(&frame);
    frame.set_label_xalign(0.5);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    frame.add(&vbox);
    vbox.set_margin(10);

    let core_count: u32 = core_count()?;
    let min_freq_mhz: u32 = freq_min_mhz(core_count)?;
    let max_freq_mhz: u32 = freq_max_mhz(core_count)?;
    let min_freq_ghz = mhz_to_ghz(min_freq_mhz);
    let max_freq_ghz = mhz_to_ghz(max_freq_mhz);

    // FREQUENCY LIMITS
    let freq_slider = gtk::Scale::with_range(gtk::Orientation::Horizontal, min_freq_ghz, max_freq_ghz, 0.1);
    vbox.add(&freq_slider);
    freq_slider.set_value(mhz_to_ghz(current_freq_max_mhz(core_count)?));

    let apply_button = gtk::Button::with_label("Set frequency limit");
    vbox.add(&apply_button);
    apply_button.connect_clicked(clone!(@weak freq_slider => move |_| {
        set_max_freq_ghz(freq_slider.value());
    }));

    let update_slider = clone!(@weak freq_slider => move || {
        freq_slider.clear_marks();
        let current_max_freq = current_freq_max_mhz(core_count).expect("Failed to get current max frequency");
        freq_slider.add_mark(min_freq_ghz, gtk::PositionType::Top, Some(format!("Min {} GHz", min_freq_ghz).as_str()));
        freq_slider.add_mark(max_freq_ghz, gtk::PositionType::Top, Some(format!("Max {} GHz", max_freq_ghz).as_str()));
        freq_slider.add_mark(mhz_to_ghz(current_max_freq), gtk::PositionType::Bottom, Some(format!("Current limit at {}GHz", mhz_to_ghz(round_to_100mhz(current_max_freq))).as_str()));
    });
    update_slider();
    glib::timeout_add_local(Duration::from_millis(1000), move || {
        update_slider();
        Continue(true)
    });

    // CURRENT FREQUENCY
    let current_frequency_bar = gtk::ProgressBar::new();
    vbox.add(&current_frequency_bar);
    current_frequency_bar.set_show_text(true);
    let current_frequency_update = move || {
        if let (Some(current_freq), Some(current_max_freq)) = (current_freq_avg_mhz(core_count), current_freq_max_mhz(core_count)) {
            current_frequency_bar.set_fraction(current_freq as f64 / current_max_freq as f64);
            current_frequency_bar.set_text(Some(format!("Current frequency {:.2} GHz", mhz_to_ghz(current_freq)).as_str()));
        }
        else {
            current_frequency_bar.set_text(Some("Failed to fetch current cpu frequency"));
        };

        Continue(true)
    };
    glib::timeout_add_local(Duration::from_millis(250), current_frequency_update);

    // CURRENT TEMP
    let current_temp_bar = gtk::ProgressBar::new();
    current_temp_bar.set_show_text(true);
    vbox.add(&current_temp_bar);
    let current_temp_update = move || {
        const MAX_TEMP: u32 = 100;
        if let Some(temp) = temperature() {
            current_temp_bar.set_fraction(min(MAX_TEMP, temp) as f64 / MAX_TEMP as f64);
            current_temp_bar.set_text(Some(format!("Current temperature {} °C", temp).as_str()));
        }
        else {
            current_temp_bar.set_text(Some("Failed to fetch current cpu temperature"));
        };

        Continue(true)
    };
    glib::timeout_add_local(Duration::from_millis(250), current_temp_update);

    Some(())
}