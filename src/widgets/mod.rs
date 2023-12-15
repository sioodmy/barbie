use gtk::{traits::*, *};

pub mod battery;
pub mod brightness;
pub mod clock;
pub mod hyprland;
pub mod power;
pub mod search;
pub mod sys;
pub mod weather;

pub fn widget() -> Box {
    let widgetbox = Box::new(Orientation::Horizontal, 0);
    widgetbox.set_widget_name("widget");
    widgetbox
}
