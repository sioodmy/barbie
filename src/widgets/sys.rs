use anyhow::Result;
use glib::*;
use gtk::{traits::*, *};
use std::thread;
use std::time::Duration;
use systemstat::{Platform, System};

use super::widget;

type Stats = (f32, f32);

pub fn add_widget(pos: &Box) {
    let widgetbox = widget();
    pos.add(&widgetbox);

    let cpulabel = Label::new(None);
    cpulabel.set_widget_name("cpu");

    let memlabel = Label::new(None);
    memlabel.set_widget_name("mem");

    let (sender, receiver) = async_channel::unbounded::<Stats>();

    widgetbox.add(&cpulabel);
    widgetbox.add(&memlabel);

    gio::spawn_blocking(move || loop {
        sender
            .send_blocking(get_stats().expect("Error while fetching system stats"))
            .unwrap();
        thread::sleep(Duration::from_secs(3));
    });

    glib::spawn_future_local(clone!(@weak cpulabel, @weak memlabel=> async move {
        while let Ok((cpu, mem)) = receiver.recv().await {
            cpulabel.set_label(&format!("  {:.2}%", cpu));
            memlabel.set_label(&format!("  {:.2}%", mem));

        }
    }));
}

fn get_stats() -> Result<Stats> {
    let sys = System::new();

    let cpu = sys.cpu_load_aggregate()?;

    thread::sleep(Duration::from_secs(1));
    let cpu = cpu.done().unwrap();

    let mem = sys.memory()?;
    let mem_percent =
        (mem.total.as_u64() - mem.free.as_u64()) as f32 / mem.total.as_u64() as f32 * 100.;

    Ok((cpu.user * 100., mem_percent))
}
