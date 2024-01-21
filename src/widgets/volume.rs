use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};
use log::error;

use std::os::unix::net::{UnixListener};
use std::process::Command;



use super::widget;

enum Volume {
    Mute,
    Unmute(i8),
}

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let label = Label::new(Some("vol"));
    widgetbox.add(&label);
    label.set_widget_name("volume");

    let (sender, receiver) = async_channel::unbounded::<Volume>();

    glib::spawn_future_local(clone!(@weak label => async move {
        while let Ok(volume) = receiver.recv().await {
            match volume {
                Volume::Mute=> {
                    label.set_label("mute");
                },
                Volume::Unmute(volume)=> {
                    label.set_label(&format!("VOL {}%", volume));
                }
            }

        }
    }));
    gio::spawn_blocking(move || loop {
        let socket = "/tmp/barbie-vol.sock";
        let _ = std::fs::remove_file(&socket);

        let listener = UnixListener::bind(socket).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(_) => {
                    sender
                        .send_blocking(get_volume().expect("couldnt get volume"))
                        .expect("couldnt send");
                }
                Err(err) => {
                    error!("Problem while parsing socket data: {}", err);
                    break;
                }
            }
        }
    });
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
