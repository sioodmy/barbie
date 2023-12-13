use anyhow::Result;
use gtk::{traits::*, *};
use std::fs;

use super::widget;

fn get_battery() -> Result<String> {
    let cap = fs::read_to_string("/sys/class/power_supply/BAT0/capacity")?
        .trim()
        .parse::<u8>()?;

    let mut icon = match cap {
        0..=10 => "󰁺",
        11..=20 => "󰁻",
        21..=30 => "󰁼",
        31..=40 => "󰁽",
        41..=50 => "󰁾",
        51..=60 => "󰁿",
        61..=70 => "󰂀",
        71..=80 => "󰂁",
        81..=90 => "󰂂",
        91..=100 => "󰁹",
        _ => "󰁹",
    }
    .to_owned();
    let status = fs::read_to_string("/sys/class/power_supply/BAT0/status")?;

    if status.trim().eq_ignore_ascii_case("Charging") {
        icon += " 󰚥";
    }
    Ok(format!("{} {}%", icon, cap))
}
pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    let initial = get_battery()?;
    let battery = Label::new(Some(&initial));
    battery.set_widget_name("battery");
    pos.add(&widgetbox);
    widgetbox.add(&battery);

    let tick = move || {
        // We can relatively safely use expect, because it should never reutrn
        // an error in real life scenario, only if you disconnect battery while
        // using the device (you shouldnt do that tbh).
        // We don't want to waste resources ticking battery widget for no reason.
        let bat = get_battery().expect("did you just disconnect the battery?");
        battery.set_text(&bat);

        // we could return glib::ControlFlow::Break to stop our battery after this tick
        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
    Ok(())
}
