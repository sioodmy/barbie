#![feature(async_closure)]
use gtk::gdk::*;
use gtk::glib::Propagation;
use gtk::prelude::*;

use gtk::*;
use gtk_layer_shell::{Edge, Layer, LayerShell};
use log::info;

mod ui;
mod widgets;

pub fn build_widgets(window: &ApplicationWindow) {
    info!("Initializing widgets");
    let root = Box::new(Orientation::Horizontal, 0);
    let left = Box::new(Orientation::Horizontal, 0);
    let centered = Box::new(Orientation::Horizontal, 0);
    let right = Box::new(Orientation::Horizontal, 0);

    root.set_widget_name("root");

    left.set_widget_name("left");
    centered.set_widget_name("centered");
    right.set_widget_name("right");

    root.set_center_widget(Some(&centered));
    root.pack_end(&right, false, true, 0);
    root.add(&left);
    window.add(&root);

    window.show_all();
}

/// Initializes the status bar.
fn activate(application: &Application) {
    let window = ApplicationWindow::new(application);
    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    LayerShell::init_layer_shell(&window);
    LayerShell::set_layer(&window, Layer::Top);
    LayerShell::auto_exclusive_zone_enable(&window);

    let display = Display::default().expect("couldnt get display");
    let monitor = display.monitor(0).expect("couldnt get monitor");

    let anchors = [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ];
    for (anchor, state) in anchors {
        LayerShell::set_anchor(&window, anchor, state);
    }

    LayerShell::set_namespace(&window, "gtk-layer-shell");

    LayerShell::set_monitor(&window, &monitor);
    window.set_app_paintable(true);

    ui::display_widgets(&window);
}

/// Applies custom visuals.
fn set_visual(window: &ApplicationWindow, screen: Option<&Screen>) {
    if let Some(screen) = screen {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // Needed for transparency, not available in GTK 4+ so
                                             // F.
        }
    }
}

/// Draws the window using a custom color and opacity.
fn draw(_: &ApplicationWindow, ctx: &cairo::Context) -> Propagation {
    let r = 24. / 255.;
    let g = 24. / 255.;
    let b = 37. / 255.;
    let a = 0.9;

    // Apply
    ctx.set_source_rgba(r, g, b, a);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("couldnt draw");

    Propagation::Proceed
}

fn load_scss() {
    let provider = CssProvider::new();
    let (style, _) = turf::inline_style_sheet!("assets/style.scss");

    provider
        .load_from_data(style.as_bytes())
        .expect("couldnt load css");

    StyleContext::add_provider_for_screen(
        &Screen::default().expect("couldnt get default screen"),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting CrabPulsar");

    color_eyre::config::HookBuilder::default()
        .display_location_section(false)
        .panic_section("something broke :3")
        .display_env_section(false)
        .install()
        .unwrap();

    let app = Application::builder()
        .application_id("dev.sioodmy.crabpulsar")
        .build();

    app.connect_startup(|_| load_scss());

    app.connect_activate(move |app| {
        activate(app);
    });

    app.run();
}
