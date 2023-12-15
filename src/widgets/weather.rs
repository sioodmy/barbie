use glib::*;
use gtk::{traits::*, *};
use log::trace;
use reqwest::{blocking, header};

use std::{thread, time::Duration};

use super::widget;

const URL: &str = "http://wttr.in?format=%c+%t";
const INTERVAL: u64 = 600;

pub fn add_widget(pos: &Box) {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let label = Label::new(None);
    widgetbox.add(&label);

    let (sender, receiver) = async_channel::unbounded::<Result<String, reqwest::Error>>();

    gio::spawn_blocking(move || loop {
        let request = get_weather();
        sender.send_blocking(request).unwrap();
        thread::sleep(Duration::from_secs(INTERVAL));
    });

    glib::spawn_future_local(clone!(@weak label, @weak widgetbox=> async move {
        while let Ok(response) = receiver.recv().await {
            match response {
                Ok(weather) => {
                    widgetbox.set_visible(true);
                    label.set_label(&weather)
                }
                Err(_) => {
                    trace!("Weather not available");
                    widgetbox.set_visible(false)
                }
            }
        }
    }));
}

fn get_weather() -> Result<String, reqwest::Error> {
    trace!("Fetching weather");
    blocking::Client::new()
        .get(URL)
        .header(header::USER_AGENT, "curl")
        .send()?
        .text()
}
