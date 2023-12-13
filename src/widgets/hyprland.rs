use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};
use hyprland::data::Workspace;
use hyprland::dispatch::*;
use hyprland::prelude::*;

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

    let update = move || {
        let id = Workspace::get_active().unwrap().id;
        if active != id {
            workspace_buttons[active as usize - 1].set_widget_name("workspace-button");
        }
        active = id;
        workspace_buttons[id as usize - 1].set_widget_name("workspace-button-active");
        ControlFlow::Continue
    };

    glib::timeout_add_seconds_local(1, update);

    Ok(())
}
