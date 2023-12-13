use gtk::{traits::*, *};

pub mod battery;
pub mod brightness;
pub mod clock;
pub mod hyprland;
pub mod search;

pub fn widget() -> Box {
    let widgetbox = Box::new(Orientation::Horizontal, 0);
    widgetbox.set_widget_name("widget");
    widgetbox
}
