use anyhow::Result;
use crossbeam_channel::unbounded;
use glib::*;
use gtk::{traits::*, *};
use hyprland::dispatch::*;
use hyprland::event_listener::*;
use hyprland::shared::WorkspaceType;
use std::thread;

use super::widget;

// TODO: rewrite all of this
// yandere dev type shit, but it works
// (for now)

fn switch_to_workspace(id: i32) {
    hyprland::dispatch!(Workspace, WorkspaceIdentifierWithSpecial::Id(id))
        .expect("Couldn't switch to workspace");
}

pub fn add_widget(pos: &Box) -> Result<()> {
    // let icons = ["󰪃", "󰩾", "󰪁", "󰪂", "󰪇", "󰪆", "󰩽", "󰩿", "󰪄", "󰪈"];
    let main_context = glib::MainContext::default();
    // ... and make it the main context by default so that we can then have a channel to send the
    // commands we received from the terminal.
    let _guard = main_context.acquire().unwrap();

    // Build the channel to get the terminal inputs from a different thread.
    let (tx, rx) = unbounded::<String>();

    let icons = ["󰪃", "󰩾", "󰪁", "󰪂", "󰪇"];
    let widgetbox = widget();
    pos.add(&widgetbox);
    let mut workspace_buttons = Vec::new();
    let mut active: i32 = 1;

    for (i, icon) in icons.iter().enumerate() {
        let button = Button::with_label(icon);
        button.connect_clicked(move |_| switch_to_workspace(i as i32 + 1));
        button.set_widget_name("workspace-button");
        widgetbox.add(&button);
        workspace_buttons.push(button);
    }

    let tick = move || {
        if let Ok(workspace) = rx.recv() {
            let id = workspace
                .parse::<i32>()
                .expect("we assumed workspace name is same as id");
            if active != id {
                workspace_buttons[active as usize - 1].set_widget_name("workspace-button");
            }
            active = id;
            workspace_buttons[id as usize - 1].set_widget_name("workspace-button-active");
        }
        ControlFlow::Continue
    };

    thread::spawn(move || {
        let mut listener = EventListener::new();
        listener.add_workspace_change_handler(move |data| {
            if let WorkspaceType::Regular(workspace) = data {
                tx.send(workspace).unwrap();
            }
        });
        listener.start_listener().unwrap();
    });

    glib::idle_add_local(tick);

    Ok(())
}
