use std::cmp::min;
use std::time::Duration;
use gtk::{prelude::*, glib, Frame};
use gtk::glib::clone;
use crate::cpu_info::{available_scaling_frequencies, core_count, current_freq_avg_mhz, get_current_governor};
use crate::cpu_info::current_freq_max_mhz;
use crate::cpu_info::freq_min_mhz;
use crate::cpu_info::freq_max_mhz;
use crate::cpu_info::temperature;
use crate::restrict::set_max_freq_ghz;
use crate::freq_util::{ghz_to_mhz, mhz_to_ghz, round_to_100mhz};

pub(crate) fn layout_cpu() -> Option<Frame> {
    let frame = gtk::Frame::new(Some("CPU"));
    frame.set_label_xalign(0.5);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    frame.add(&vbox);
    vbox.set_margin(10);

    let core_count: u32 = core_count()?;
    let min_freq_mhz: u32 = freq_min_mhz(core_count)?;
    let max_freq_mhz: u32 = freq_max_mhz(core_count)?;
    let min_freq_ghz = mhz_to_ghz(min_freq_mhz);
    let max_freq_ghz = mhz_to_ghz(max_freq_mhz);
    let selected_freq = mhz_to_ghz(current_freq_max_mhz(core_count)?);

    let governor_label = gtk::Label::new(None);
    vbox.add(&governor_label);

    // FREQUENCY LIMITS
    let freq_slider = gtk::Scale::with_range(gtk::Orientation::Horizontal, min_freq_ghz, max_freq_ghz, 0.1);
    vbox.add(&freq_slider);
    freq_slider.set_value(selected_freq);

    let apply_button = gtk::Button::with_label("Set frequency limit");
    vbox.add(&apply_button);
    apply_button.connect_clicked(clone!(@weak freq_slider => move |_| {
        if let Err(error) = set_max_freq_ghz(freq_slider.value()) {
            eprintln!("Failed to set frequency limit: {}", error);
        }
    }));

    freq_slider.connect_change_value(|_slider, _scroll_type, value| {
        let freq_is_available = if let Some(freqs) = available_scaling_frequencies() {
            freqs.contains(&round_to_100mhz(ghz_to_mhz(value)))
        }
        // Assume all frequencies are available if no configuration is found
        else { true };
        return Inhibit(!freq_is_available);
    });

    let update_slider = clone!(@weak freq_slider => move || {
        freq_slider.clear_marks();
        let current_max_freq = current_freq_max_mhz(core_count).expect("Failed to get current max frequency");
        freq_slider.add_mark(min_freq_ghz, gtk::PositionType::Top, Some(format!("\nMin {:.2} GHz", min_freq_ghz).as_str()));
        freq_slider.add_mark(max_freq_ghz, gtk::PositionType::Top, Some(format!("\nMax {:.2} GHz", max_freq_ghz).as_str()));
        if let Some(freqs) = available_scaling_frequencies() {
            for freq in freqs {
                let ghz = mhz_to_ghz(freq);
                let offset = ghz + 0.001;
                freq_slider.add_mark(offset, gtk::PositionType::Bottom, Some(format!("{:.1}", ghz).as_str()));
            }
        }
        let current_governor = get_current_governor().expect("Failed to get current governor");
        governor_label.set_text(format!("CPU governor: {}", current_governor).as_str());
        freq_slider.add_mark(mhz_to_ghz(current_max_freq), gtk::PositionType::Top, Some(format!("Current limit at {:.1}GHz", mhz_to_ghz(round_to_100mhz(current_max_freq))).as_str()));
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
        match temperature() {
            Ok(temp) => {
                current_temp_bar.set_fraction(min(MAX_TEMP, temp) as f64 / MAX_TEMP as f64);
                current_temp_bar.set_text(Some(format!("Current temperature {} °C", temp).as_str()));
            },
            Err(error) => {
                current_temp_bar.set_text(Some(error));
            }
        };

        Continue(true)
    };
    glib::timeout_add_local(Duration::from_millis(250), current_temp_update);

    Some(frame)
}
