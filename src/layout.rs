use crate::layout_cpu::layout_cpu;
use crate::layout_ram::layout_ram;

pub fn build_layout() -> Option<gtk::Box> {
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 15);

    layout_cpu(&layout);
    layout_ram(&layout);

    return Some(layout);
}