use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};
use hyprland::data::Workspace;

use hyprland::event_listener::*;
use hyprland::prelude::*;

use super::widget;

const WORKSPACES: usize = 5;
const SEPARATOR: &str = "   ";

struct WorkspacesData {
    icons: [String; WORKSPACES],
    active: i32,
}

impl WorkspacesData {
    fn gen_markup(&self) -> String {
        let mut markup = String::new();
        for (i, icon) in self.icons.iter().enumerate() {
            if i != 0 {
                markup.push_str(SEPARATOR);
            }
            markup.push_str(if self.active as usize == i + 1 {
                "<span color = \"#f9e2af\">"
            } else {
                "<span color = \"#89b4fa\">"
            });
            markup.push_str(icon);
            markup.push_str("</span>");
        }

        markup
    }
}

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let (sender, receiver) = async_channel::unbounded::<()>();

    let label = Label::new(None);
    let mut wdata = WorkspacesData {
        icons: ["󰪃".into(), "󰩾".into(), "󰪁".into(), "󰪂".into(), "󰪇".into()],
        active: 1,
    };

    label.set_markup(&wdata.gen_markup());
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
            wdata.active =
                Workspace::get_active()
                .expect("couldnt get active workspace").id;
            label.set_markup(&wdata.gen_markup());
        }
    }));

    Ok(())
}
