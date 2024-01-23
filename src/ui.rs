use gtk::traits::*;
use gtk::*;
use log::warn;

use crate::widgets::*;

macro_rules! add {
    ($widget:ident to $pos:ident) => {
        match $widget::add_widget(&$pos) {
            Ok(_) => (),
            Err(_) => warn!("Couldn't load {} widget", stringify!($widget)),
        }
    };
}
pub fn display_widgets(window: &ApplicationWindow) {
    let root = Box::new(Orientation::Horizontal, 0);
    root.set_widget_name("barbie");

    let left = widget();
    let center = widget();
    let right = widget();

    left.set_widget_name("widget");
    center.set_widget_name("widget");
    right.set_widget_name("widget");

    root.add(&left);
    root.set_center_widget(Some(&center));
    root.pack_end(&right, false, true, 0);

    add!(hyprland to left);
    add!(battery to left);
    add!(brightness to left);

    add!(clock to center);

    add!(sys to right);
    add!(volume to right);

    window.add(&root);
    window.show_all();
}
