use gtk::*;

pub mod battery;
pub mod brightness;
pub mod clock;
pub mod hyprland;
pub mod sys;
pub mod volume;

pub fn widget() -> Box {
    let widgetbox = Box::new(Orientation::Horizontal, 20);
    widgetbox
}
