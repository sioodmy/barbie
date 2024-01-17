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

    let label = Label::new(Some(&Workspace::get_active()?.id.to_string()));

    label.set_widget_name("hyprland");
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
            let active =
                Workspace::get_active()
                .expect("couldnt get active workspace").id;
            label.set_label(&format!("{}", active));
        }
    }));

    Ok(())
}
