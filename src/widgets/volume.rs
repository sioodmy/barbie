use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};
use std::process::Command;
use std::thread;
use std::time::Duration;

use super::widget;

const MUTE: i8 = -1;
pub fn add_widget(pos: &Box) {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let label = Label::new(None);
    label.set_widget_name("volume");

    let (sender, receiver) = async_channel::unbounded::<i8>();
    // TODO: write actual unix socket
    glib::spawn_future_local(clone!(@weak label => async move {
        println!("opening channel");
        while let Ok(volume) = receiver.recv().await {
            match volume {
                MUTE => {
                    label.set_label("mute");
                },
                _ => {
                    label.set_label(&format!("{}%", volume));
                }
            }

        }
    }));
    gio::spawn_blocking(move || {
        thread::sleep(Duration::from_secs(3));
        // sender
        //     .send_blocking(get_volume().expect("couldnt get volume"))
        //     .expect("couldnt send");
        match sender.send_blocking(12) {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        };
    });
}

fn get_volume() -> Result<i8> {
    let mute = String::from_utf8(
        Command::new("pamixer")
            .args(["--get-mute"])
            .output()
            .expect("failed to execute pamixer")
            .stdout,
    )?;

    if mute.trim().eq_ignore_ascii_case("true") {
        return Ok(MUTE);
    }

    let volume = String::from_utf8(
        Command::new("pamixer")
            .args(["--get-volume"])
            .output()
            .expect("failed to execute pamixer")
            .stdout,
    )?;

    Ok(volume.trim().parse::<i8>().unwrap())
}
