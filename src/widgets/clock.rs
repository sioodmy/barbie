use anyhow::Result;
use chrono::Local;
use gtk::{traits::*, *};

use super::widget;

fn current_time() -> String {
    format!("{}", Local::now().format("%H:%M"))
}

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);
    let clock = Label::new(None);

    clock.set_widget_name("clock");
    widgetbox.add(&clock);

    let tick = move || {
        let time = current_time();
        clock.set_text(&time);
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
    Ok(())
}
