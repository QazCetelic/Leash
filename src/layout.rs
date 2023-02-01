use std::cell::RefCell;
use std::cmp::min;
use std::rc::Rc;
use std::time::Duration;

use gtk::{prelude::*, glib};
use crate::cpu_info::current_freq_avg_mhz;
use crate::cpu_info::current_freq_max_mhz;
use crate::cpu_info::freq_min_mhz;
use crate::cpu_info::freq_max_mhz;
use crate::cpu_info::temperature;
use crate::cpu_restrict::set_max_freq_ghz;
use crate::freq_util::{mhz_to_ghz, round_to_100mhz};

pub fn build_layout() -> Option<gtk::Box> {
    let min_freq_mhz: u32 = freq_min_mhz()?;
    let max_freq_mhz: u32 = freq_max_mhz()?;
    let min_freq_ghz = mhz_to_ghz(min_freq_mhz);
    let max_freq_ghz = mhz_to_ghz(max_freq_mhz);

    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // FREQUENCY LIMITS
    let freq_slider = gtk::Scale::with_range(gtk::Orientation::Horizontal, min_freq_ghz, max_freq_ghz, 0.1);
    freq_slider.set_value(mhz_to_ghz(current_freq_max_mhz()?));
    layout.add(&freq_slider);
    let freq_slider_rc = Rc::new(RefCell::new(freq_slider));

    let freq_slider_apply = freq_slider_rc.clone();
    let apply_button = gtk::Button::with_label("Set limit");
    apply_button.connect_clicked(move |_| {
        set_max_freq_ghz(freq_slider_apply.borrow().value());
    });
    layout.add(&apply_button);

    let freq_slider_update = freq_slider_rc.clone();
    let update_slider = move || {
        let mut_borrow = freq_slider_update.borrow_mut();
        mut_borrow.clear_marks();
        let current_max_freq = current_freq_max_mhz().expect("Failed to get current max frequency");
        mut_borrow.add_mark(min_freq_ghz, gtk::PositionType::Top, Some(format!("Min {} GHz", min_freq_ghz).as_str()));
        mut_borrow.add_mark(max_freq_ghz, gtk::PositionType::Top, Some(format!("Max {} GHz", max_freq_ghz).as_str()));
        mut_borrow.add_mark(mhz_to_ghz(current_max_freq), gtk::PositionType::Bottom, Some(format!("Current limit at {}GHz", mhz_to_ghz(round_to_100mhz(current_max_freq))).as_str()));

        Continue(true)
    };
    glib::timeout_add_local(Duration::from_millis(500), update_slider);

    // CURRENT FREQUENCY
    let current_frequency_bar = gtk::ProgressBar::new();
    current_frequency_bar.set_show_text(true);
    layout.add(&current_frequency_bar);
    let current_frequency_update = move || {
        if let (Some(current_freq), Some(current_max_freq)) = (current_freq_avg_mhz(), current_freq_max_mhz()) {
            current_frequency_bar.set_fraction(current_freq as f64 / current_max_freq as f64);
            current_frequency_bar.set_text(Some(format!("Current frequency {:.2} GHz", mhz_to_ghz(current_freq)).as_str()));
        }
        else {
            current_frequency_bar.set_text(Some("Failed to fetch current cpu frequency"));
        };

        Continue(true)
    };
    glib::timeout_add_local(Duration::from_millis(100), current_frequency_update);

    // CURRENT TEMP
    let current_temp_bar = gtk::ProgressBar::new();
    current_temp_bar.set_show_text(true);
    layout.add(&current_temp_bar);
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
    glib::timeout_add_local(Duration::from_millis(100), current_temp_update);

    return Some(layout);
}