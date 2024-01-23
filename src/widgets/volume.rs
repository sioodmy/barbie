use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};

use std::process::Command;

use crate::socket::make_socket;

use super::widget;

enum Volume {
    Mute,
    Unmute(i8),
}

fn vol_label(volume: Volume) -> String {
    match volume {
        Volume::Mute => String::from(" 0%"),
        Volume::Unmute(volume) => {
            let icon = match volume {
                1..=50 => "",
                51..=100 | _ => "",
            };
            format!("{} {}%", icon, volume)
        }
    }
}
pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let initial = vol_label(get_volume().unwrap());
    let label = Label::new(Some(&initial));
    widgetbox.add(&label);
    label.set_widget_name("volume");

    let (sender, receiver) = async_channel::unbounded::<()>();

    glib::spawn_future_local(clone!(@weak label => async move {
        while let Ok(()) = receiver.recv().await {
            label.set_label(&vol_label(get_volume().unwrap()))
        }
    }));

    make_socket("vol", sender);
    Ok(())
}

fn get_volume() -> Result<Volume> {
    let mute = String::from_utf8(
        Command::new("pamixer")
            .args(["--get-mute"])
            .output()
            .expect("failed to execute pamixer")
            .stdout,
    )?;

    if mute.trim().eq_ignore_ascii_case("true") {
        return Ok(Volume::Mute);
    }

    let volume = String::from_utf8(
        Command::new("pamixer")
            .args(["--get-volume"])
            .output()
            .expect("failed to execute pamixer")
            .stdout,
    )?;

    let value = volume.trim().parse::<i8>().unwrap();
    Ok(Volume::Unmute(value))
}
