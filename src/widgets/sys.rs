use anyhow::Result;
use core::f32;
use glib::*;
use gtk::{traits::*, *};
use rand::prelude::SliceRandom;
use std::thread;
use std::time::Duration;
use systemstat::{Platform, System};

use super::widget;

type Stats = (f32, f32);

macro_rules! pick {
    ($($x:expr),+ $(,)?) => {
        vec![$($x),+].choose(&mut rand::thread_rng()).unwrap()
    };
}
fn tamagotchi(cpu: f32) -> String {
    match cpu as i32 {
        0..=10 => pick!("( ◕‿◕)", "(◕‿◕ )", "(≧◡≦)"),
        // 0..=10 => ,
        11..=20 => "(◕‿◕)",
        21..=30 => "(• ᴗ •)",
        31..=50 => "(￣︿￣)",
        51..=80 => "(︶︹︺)",
        81..=101 => "(☓‿‿☓)",
        _ => "a",
    }
    .to_string()
}

pub fn add_widget(pos: &Box) -> Result<()> {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let face = Label::new(None);
    face.set_widget_name("face");
    face.set_width_request(74);

    let (sender, receiver) = async_channel::unbounded::<f32>();

    widgetbox.add(&face);

    gio::spawn_blocking(move || loop {
        sender
            .send_blocking(get_stats().expect("Error while fetching system stats"))
            .unwrap();
        thread::sleep(Duration::from_secs(3));
    });

    glib::spawn_future_local(clone!(@weak face=> async move {
        while let Ok(cpu) = receiver.recv().await {
            face.set_label(&tamagotchi(cpu));

        }
    }));
    Ok(())
}

fn get_stats() -> Result<f32> {
    let sys = System::new();

    let cpu = sys.cpu_load_aggregate()?;

    thread::sleep(Duration::from_secs(1));
    let cpu = cpu.done().unwrap();

    Ok(cpu.user * 100.)
}
