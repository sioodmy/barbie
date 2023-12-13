use gtk::{traits::*, *};

use super::widget;

pub fn add_widget(pos: &Box) {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let cpu = Label::new(Some("  3% "));
    cpu.set_widget_name("cpu");

    let mem = Label::new(Some(" 6.3%"));
    mem.set_widget_name("mem");

    widgetbox.add(&cpu);
    widgetbox.add(&mem);
}
