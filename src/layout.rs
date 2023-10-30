use gtk::prelude::BoxExt;
use crate::layout_cpu::layout_cpu;
use crate::layout_ram::layout_ram;

pub fn build_layout() -> Option<gtk::Box> {
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 10);

    layout.pack_start(&layout_cpu()?, false, false, 0);
    layout.pack_end(&layout_ram()?, false, false, 0);

    return Some(layout);
}
