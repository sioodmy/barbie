#![feature(async_closure)]
use gtk::gdk::*;
use gtk::glib::Propagation;
use gtk::prelude::*;

use gtk::*;
use gtk_layer_shell::{Edge, Layer, LayerShell};

mod ui;
mod widgets;

fn get_anchors() -> [(gtk_layer_shell::Edge, bool); 4] {
    let expand_left = true;
    let expand_right = true;

    // If the position was valid, return the result.
    [
        (Edge::Left, expand_left),
        (Edge::Right, expand_right),
        (Edge::Top, true),
        (Edge::Bottom, false),
    ]
}

// Builds all of the widgets.
pub fn build_widgets(window: &ApplicationWindow) {
    // Create box widgets, which we'll be using to draw the content onto.
    let root = Box::new(Orientation::Horizontal, 0);
    let left = Box::new(Orientation::Horizontal, 0);
    let centered = Box::new(Orientation::Horizontal, 0);
    let right = Box::new(Orientation::Horizontal, 0);

    // 0.2.5: Root expands across the entire bar, previously "left" would do this but it isn't
    //   ideal when customizing, since borders would draw on the entire bar rather than just on the
    //   left portion of the bar.
    root.set_widget_name("root");

    // 0.2.5: Allow for customizing left, centered and right.
    left.set_widget_name("left");
    centered.set_widget_name("centered");
    right.set_widget_name("right");

    root.set_center_widget(Some(&centered));
    root.pack_end(&right, false, true, 0);
    root.add(&left);
    window.add(&root);

    // Prepare and show all of the widgets.
    window.show_all();

    // Update dynamic content.
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

    for (anchor, state) in get_anchors() {
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
    let app = Application::builder()
        .application_id("dev.sioodmy.crabpulsar")
        .build();

    app.connect_startup(|_| load_scss());

    app.connect_activate(move |app| {
        activate(app);
    });

    app.run();
}
