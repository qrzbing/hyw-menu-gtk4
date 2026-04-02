use std::cell::RefCell;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

use gtk4::prelude::*;
use gtk4::{
    Align, ApplicationWindow, Box as GtkBox, Button, GestureClick, Grid, Justification, Label,
    Orientation, ScrolledWindow,
};
use serde::{Deserialize, Serialize};

use crate::app::catalog::{DesktopAppCatalog, DesktopAppEntry};
use crate::app::tiles::{attach_launch_handler, build_app_tile_widget};
use crate::config::LauncherConfig;

#[derive(Debug, Default, Serialize, Deserialize)]
struct QuickAccessFile {
    #[serde(default)]
    apps: Vec<String>,
}

#[derive(Clone)]
pub(crate) struct QuickAccessState {
    path: PathBuf,
    catalog: DesktopAppCatalog,
    config: Arc<LauncherConfig>,
    window: ApplicationWindow,
    app_ids: Rc<RefCell<Vec<String>>>,
    container: Rc<RefCell<Option<GtkBox>>>,
}

impl QuickAccessState {
    pub(crate) fn new(
        path: PathBuf,
        catalog: DesktopAppCatalog,
        config: Arc<LauncherConfig>,
        window: &ApplicationWindow,
    ) -> Self {
        let app_ids = Self::load_ids(&path, &catalog);

        Self {
            path,
            catalog,
            config,
            window: window.clone(),
            app_ids: Rc::new(RefCell::new(app_ids)),
            container: Rc::new(RefCell::new(None)),
        }
    }

    pub(crate) fn build_home_page(&self) -> GtkBox {
        let page = GtkBox::new(Orientation::Vertical, 0);
        page.add_css_class("launcher-page");
        page.add_css_class("launcher-home-page");
        page.set_hexpand(true);
        page.set_vexpand(true);

        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_hexpand(true);
        container.set_vexpand(true);

        *self.container.borrow_mut() = Some(container.clone());
        self.refresh();

        page.append(&container);
        page
    }

    pub(crate) fn add_app(&self, app_id: &str) {
        if self.catalog.get(app_id).is_none() {
            return;
        }

        let mut app_ids = self.app_ids.borrow_mut();
        if app_ids.iter().any(|existing| existing == app_id) {
            return;
        }

        app_ids.push(app_id.to_string());
        if let Err(error) = self.save_ids(&app_ids) {
            eprintln!("Failed to save quick access: {error}");
        }
        drop(app_ids);
        self.refresh();
    }

    fn remove_app(&self, app_id: &str) {
        let mut app_ids = self.app_ids.borrow_mut();
        let original_len = app_ids.len();
        app_ids.retain(|existing| existing != app_id);

        if app_ids.len() == original_len {
            return;
        }

        if let Err(error) = self.save_ids(&app_ids) {
            eprintln!("Failed to save quick access: {error}");
        }
        drop(app_ids);
        self.refresh();
    }

    fn refresh(&self) {
        let Some(container) = self.container.borrow().clone() else {
            return;
        };

        while let Some(child) = container.first_child() {
            container.remove(&child);
        }

        if self.app_ids.borrow().is_empty() {
            container.append(&self.build_empty_state());
        } else {
            container.append(&self.build_scroller());
        }
    }

    fn build_empty_state(&self) -> GtkBox {
        let box_ = GtkBox::new(Orientation::Vertical, 0);
        let label = Label::new(Some("Right-click apps in All Apps to add them here"));

        box_.add_css_class("launcher-empty-state");
        box_.set_hexpand(true);
        box_.set_vexpand(true);
        box_.set_halign(Align::Fill);
        box_.set_valign(Align::Fill);

        label.add_css_class("launcher-empty-state-label");
        label.set_wrap(true);
        label.set_justify(Justification::Center);
        label.set_halign(Align::Center);
        label.set_valign(Align::Center);

        box_.append(&label);
        box_
    }

    fn build_scroller(&self) -> ScrolledWindow {
        let scroller = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();
        scroller.add_css_class("launcher-grid-scroller");

        let grid = Grid::new();
        let spacing = self.config.grid().tile_spacing().max(0) as u32;
        grid.set_column_spacing(spacing);
        grid.set_row_spacing(spacing);
        grid.set_halign(Align::Start);
        grid.set_valign(Align::Start);
        grid.set_hexpand(false);
        grid.set_vexpand(false);
        grid.set_margin_start(self.config.grid().side_margin());
        grid.set_margin_end(self.config.grid().side_margin());
        grid.add_css_class("launcher-grid-section");

        for (index, app_id) in self.app_ids.borrow().iter().enumerate() {
            if let Some(app) = self.catalog.get(app_id) {
                let column = (index as i32) % self.config.grid().columns();
                let row = (index as i32) / self.config.grid().columns();
                grid.attach(&self.build_quick_access_tile(app), column, row, 1, 1);
            }
        }

        scroller.set_child(Some(&grid));
        scroller
    }

    fn build_quick_access_tile(&self, app: &DesktopAppEntry) -> Button {
        let button = build_app_tile_widget(
            app,
            self.config.grid().tile_size(),
            self.config.grid().icon_size(),
            self.config.grid().icon_center_y_ratio(),
            self.config.grid().title_font_size(),
            self.config.grid().title_top_margin(),
            None,
        );
        attach_launch_handler(&button, app, &self.window);

        let remove_state = self.clone();
        let app_id = app.id().to_owned();
        let right_click = GestureClick::new();
        right_click.set_button(3);
        right_click.connect_pressed(move |_, _, _, _| {
            remove_state.remove_app(&app_id);
        });
        button.add_controller(right_click);

        button
    }

    fn load_ids(path: &Path, catalog: &DesktopAppCatalog) -> Vec<String> {
        let Ok(content) = fs::read_to_string(path) else {
            return Vec::new();
        };

        let Ok(file) = toml::from_str::<QuickAccessFile>(&content) else {
            return Vec::new();
        };

        let mut deduped = Vec::new();
        for app_id in file.apps {
            if catalog.get(&app_id).is_none() {
                continue;
            }

            if deduped.iter().any(|existing| existing == &app_id) {
                continue;
            }

            deduped.push(app_id);
        }

        deduped
    }

    fn save_ids(&self, app_ids: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(&QuickAccessFile {
            apps: app_ids.to_vec(),
        })?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}
