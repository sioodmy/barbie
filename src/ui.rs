use gtk::traits::*;
use gtk::*;

use crate::widgets::*;

pub fn display_widgets(window: &ApplicationWindow) {
    let root = Box::new(Orientation::Horizontal, 0);

    let right = Box::new(Orientation::Horizontal, 0);
    let left = Box::new(Orientation::Horizontal, 0);
    let center = Box::new(Orientation::Horizontal, 0);

    root.add(&left);
    root.set_center_widget(Some(&center));
    root.pack_end(&right, false, true, 0);

    search::add_widget(&left);
    match hyprland::add_widget(&left) {
        Ok(_) => (),
        Err(_) => eprintln!("couldnt load hyprland module"),
    }
    match battery::add_widget(&left) {
        Ok(_) => (),
        Err(_) => eprintln!("couldnt load battery module"),
    }
    match brightness::add_widget(&left) {
        Ok(_) => (),
        Err(_) => eprintln!("couldnt load brightness module"),
    }
    clock::add_widget(&right);

    window.add(&root);
    window.show_all();
}
