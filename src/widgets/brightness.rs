use anyhow::Result;
use gtk::{traits::*, *};
use std::fs;

use super::widget;

fn get_brightness() -> Result<String> {
    let brightness = fs::read_to_string("/sys/class/backlight/intel_backlight/brightness")?
        .trim()
        .parse::<f32>()?;
    let max_brightness = fs::read_to_string("/sys/class/backlight/intel_backlight/max_brightness")?
        .trim()
        .parse::<f32>()?;
    let percent = (brightness / max_brightness * 100.) as u32;
    let icon = match percent {
        0 => "",
        1..=15 => "",
        16..=29 => "",
        30..=43 => "",
        44..=57 => "",
        58..=71 => "",
        72..=85 => "",
        86..=99 => "",
        100 => "",
        _ => "",
    };

    Ok(format!("{} {}%", icon, percent))
}
pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);
    let initial = get_brightness()?;
    let brightness = Label::new(Some(&initial));
    brightness.set_widget_name("brightness");
    widgetbox.add(&brightness);

    let tick = move || {
        // We can relatively safely use expect, because it should never reutrn
        // an error in real life scenario, only if you disconnect battery while
        // using the device (you shouldnt do that tbh).
        // We don't want to waste resources ticking battery widget for no reason.
        let brightness_label = get_brightness().expect("failed to update brightness widget");
        brightness.set_text(&brightness_label);

        // we could return glib::ControlFlow::Break to stop our battery after this tick
        glib::ControlFlow::Continue
    };

    // executes the closure once every second
    glib::timeout_add_seconds_local(1, tick);
    Ok(())
}
