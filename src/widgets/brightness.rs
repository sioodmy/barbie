use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};
use std::fs;

use crate::socket::make_socket;

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
    let label = Label::new(Some(&initial));
    label.set_widget_name("brightness");
    widgetbox.add(&label);

    let (sender, receiver) = async_channel::unbounded::<()>();

    glib::spawn_future_local(clone!(@weak label => async move {
        while let Ok(()) = receiver.recv().await {
            label.set_label(&get_brightness().unwrap());
        }
    }));

    make_socket("bl", sender);
    Ok(())
}
