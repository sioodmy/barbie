use gtk::{traits::*, *};
use hyprland::dispatch::*;

pub fn add_widget(pos: &Box) {
    let search = Button::with_label(" ");
    search.connect_clicked(|_| {
        // TODO: actual powermenu
        hyprland::dispatch!(Exec, "anyrun").expect("Failed to open power menu");
    });
    search.set_widget_name("power");

    pos.add(&search)
}
