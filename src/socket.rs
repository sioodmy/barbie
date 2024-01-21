use async_channel::Sender;
use glib::*;
use gtk::{traits::*, *};
use log::error;
use std::os::unix::net::UnixListener;

pub fn make_socket(name: &'static str, sender: Sender<()>) {
    gio::spawn_blocking(move || loop {
        let socket = format!("/tmp/barbie-{}.sock", name);
        let _ = std::fs::remove_file(&socket);

        let listener = UnixListener::bind(socket).unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(_) => {
                    sender.send_blocking(()).expect("couldnt send");
                }
                Err(err) => {
                    error!("Problem while parsing socket data: {}", err);
                    break;
                }
            }
        }
    });
}
