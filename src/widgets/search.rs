use gtk::{traits::*, *};
use hyprland::dispatch::*;

pub fn add_widget(pos: &Box) {
    let search = Button::with_label(" ");
    search.connect_clicked(|_| {
        hyprland::dispatch!(Exec, "anyrun").expect("Failed to open anyrun");
    });
    search.set_widget_name("search");

    pos.add(&search)
}
