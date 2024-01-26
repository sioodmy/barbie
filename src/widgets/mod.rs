use gtk::*;

pub mod battery;
pub mod brightness;
pub mod clock;
pub mod hyprland;
pub mod sys;
pub mod volume;

pub fn widget() -> Box {
    Box::new(Orientation::Horizontal, 25)
}
