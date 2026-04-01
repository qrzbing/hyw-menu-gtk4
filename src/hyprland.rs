use std::env;
use std::sync::Arc;

use gtk4::gdk::{self, Display};
use gtk4::glib::object::Cast as _;
use gtk4::prelude::*;
use tokio::runtime::{Builder, Runtime};
use wayle_hyprland::HyprlandService;

#[derive(Clone)]
pub struct HyprlandContext {
    _runtime: Arc<Runtime>,
    service: Arc<HyprlandService>,
}

impl HyprlandContext {
    pub fn new() -> Option<Self> {
        if env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_none() {
            return None;
        }

        let runtime = Builder::new_multi_thread().enable_all().build().ok()?;
        let service = runtime.block_on(HyprlandService::new()).ok()?;

        Some(Self {
            _runtime: Arc::new(runtime),
            service,
        })
    }

    pub fn focused_monitor_name(&self) -> Option<String> {
        self.service
            .monitors
            .get()
            .into_iter()
            .find(|monitor| monitor.focused.get())
            .map(|monitor| monitor.name.get())
    }
}

pub fn preferred_monitor(
    display: &Display,
    hyprland: Option<&HyprlandContext>,
    min_width: i32,
) -> Option<gdk::Monitor> {
    if let Some(name) = hyprland.and_then(HyprlandContext::focused_monitor_name) {
        if let Some(monitor) = monitor_by_connector(display, &name) {
            return Some(monitor);
        }
    }

    first_suitable_monitor(display, min_width)
}

fn first_suitable_monitor(display: &Display, min_width: i32) -> Option<gdk::Monitor> {
    all_monitors(display)
        .into_iter()
        .find(|monitor| monitor.geometry().width() >= min_width)
        .or_else(|| all_monitors(display).into_iter().next())
}

fn monitor_by_connector(display: &Display, connector: &str) -> Option<gdk::Monitor> {
    all_monitors(display)
        .into_iter()
        .find(|monitor| monitor.connector().as_deref() == Some(connector))
}

fn all_monitors(display: &Display) -> Vec<gdk::Monitor> {
    let monitors = display.monitors();

    (0..monitors.n_items())
        .filter_map(|index| monitors.item(index))
        .filter_map(|obj| obj.downcast::<gdk::Monitor>().ok())
        .collect()
}
