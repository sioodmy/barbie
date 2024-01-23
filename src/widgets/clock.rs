use anyhow::Result;
use chrono::Local;
use gtk::{traits::*, *};

use super::widget;

fn current_time() -> String {
    Local::now().format("%H:%M").to_string()
}
fn current_date() -> String {
    Local::now().format("%Y-%m-%d").to_string()
}

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);
    let clock = Label::new(Some(&current_time()));
    clock.set_tooltip_text(Some(&current_date()));

    clock.set_widget_name("clock");
    widgetbox.add(&clock);

    let tick = move || {
        clock.set_text(&current_time());
        clock.set_tooltip_text(Some(&current_date()));
        // we could return glib::ControlFlow::Break to stop our clock after this tick
        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
    Ok(())
}
