use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};
use hyprland::data::Workspace;

use hyprland::event_listener::*;
use hyprland::prelude::*;

use super::widget;

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let (sender, receiver) = async_channel::unbounded::<()>();

    let label = Label::new(Some(workspace_name()));

    label.set_widget_name("hyprland");
    label.set_width_request(30);
    widgetbox.add(&label);

    gio::spawn_blocking(move || {
        let mut listener = EventListener::new();
        listener.add_workspace_change_handler(move |_| {
            sender.send_blocking(()).unwrap();
        });
        listener.start_listener().unwrap();
    });

    glib::spawn_future_local(clone!(@weak label=> async move {
        while (receiver.recv().await).is_ok() {
            label.set_label(workspace_name());
        }
    }));

    Ok(())
}

fn workspace_name() -> &'static str {
    let id = Workspace::get_active().unwrap().id;
    match id {
        1 => "I",
        2 => "II",
        3 => "III",
        4 => "IV",
        5 => "V",
        6 => "VI",
        7 => "VII",
        8 => "VII",
        9 => "IX",
        10 => "X",
        _ => todo!("support more workspaces"),
    }
}
