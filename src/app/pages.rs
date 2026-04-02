use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use gtk4::prelude::*;
use gtk4::{
    Align, ApplicationWindow, Box as GtkBox, Button, GestureClick, Grid, Label, Orientation,
    ScrolledWindow, Stack, StackTransitionType,
};

use super::catalog::{DesktopAppCatalog, DesktopAppEntry};
use super::launcher::LauncherWindow;
use super::quick_access::QuickAccessState;
use super::tiles::{attach_launch_handler, build_app_tile_widget};
use crate::config::LauncherConfig;

#[derive(Clone)]
pub(crate) struct AllAppsPageState {
    catalog: DesktopAppCatalog,
    quick_access: QuickAccessState,
    config: Arc<LauncherConfig>,
    window: ApplicationWindow,
    query: Rc<RefCell<String>>,
    container: Rc<RefCell<Option<GtkBox>>>,
}

impl AllAppsPageState {
    pub(crate) fn new(
        catalog: DesktopAppCatalog,
        quick_access: QuickAccessState,
        config: Arc<LauncherConfig>,
        window: &ApplicationWindow,
    ) -> Self {
        Self {
            catalog,
            quick_access,
            config,
            window: window.clone(),
            query: Rc::new(RefCell::new(String::new())),
            container: Rc::new(RefCell::new(None)),
        }
    }

    pub(crate) fn build_page(&self) -> GtkBox {
        let page = GtkBox::new(Orientation::Vertical, 12);
        let container = GtkBox::new(Orientation::Vertical, 12);

        page.add_css_class("launcher-page");
        page.add_css_class("launcher-all-apps-page");
        page.set_hexpand(true);
        page.set_vexpand(true);
        container.set_hexpand(true);
        container.set_vexpand(true);

        *self.container.borrow_mut() = Some(container.clone());
        self.refresh();

        page.append(&container);
        page
    }

    pub(crate) fn set_query(&self, query: &str) {
        *self.query.borrow_mut() = query.trim().to_owned();
        self.refresh();
    }

    fn refresh(&self) {
        let Some(container) = self.container.borrow().clone() else {
            return;
        };

        while let Some(child) = container.first_child() {
            container.remove(&child);
        }

        let filtered_apps = self.filtered_apps();
        container.append(&self.build_intro(filtered_apps.len()));
        container.append(&self.build_scroller(&filtered_apps));
    }

    fn build_intro(&self, result_count: usize) -> GtkBox {
        let intro = GtkBox::new(Orientation::Horizontal, 12);
        let text_column = GtkBox::new(Orientation::Vertical, 4);
        let title = Label::new(Some(self.intro_title()));
        let subtitle = Label::new(Some(&format!("showing {result_count} applications")));

        intro.add_css_class("launcher-page-intro");
        intro.set_hexpand(true);
        text_column.set_hexpand(true);

        title.add_css_class("launcher-page-title");
        title.set_xalign(0.0);
        subtitle.add_css_class("launcher-page-subtitle");
        subtitle.set_xalign(0.0);

        text_column.append(&title);
        text_column.append(&subtitle);
        intro.append(&text_column);
        intro
    }

    fn intro_title(&self) -> &str {
        if self.query.borrow().is_empty() {
            "All Applications"
        } else {
            "Search Results"
        }
    }

    fn filtered_apps(&self) -> Vec<DesktopAppEntry> {
        let query = self.query.borrow().to_lowercase();
        if query.is_empty() {
            return self.catalog.ordered().to_vec();
        }

        self.catalog
            .ordered()
            .iter()
            .filter(|app| Self::matches_query(app, &query))
            .cloned()
            .collect()
    }

    fn matches_query(app: &DesktopAppEntry, query: &str) -> bool {
        app.name().to_lowercase().contains(query)
            || app.id().to_lowercase().contains(query)
            || app
                .description()
                .map(|description| description.to_lowercase().contains(query))
                .unwrap_or(false)
    }

    fn build_scroller(&self, apps: &[DesktopAppEntry]) -> ScrolledWindow {
        let scroller = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();
        scroller.add_css_class("launcher-apps-scroller");

        let grid = Grid::new();
        let spacing = self.config.grid().spacing().max(0) as u32;
        grid.set_column_spacing(spacing);
        grid.set_row_spacing(spacing);
        grid.set_halign(Align::Start);
        grid.set_valign(Align::Start);
        grid.set_hexpand(false);
        grid.set_vexpand(false);
        grid.add_css_class("launcher-apps-grid");

        for (index, app) in apps.iter().enumerate() {
            let column = (index as i32) % self.config.grid().columns();
            let row = (index as i32) / self.config.grid().columns();
            grid.attach(&self.build_tile(app), column, row, 1, 1);
        }

        scroller.set_child(Some(&grid));
        scroller
    }

    fn build_tile(&self, app: &DesktopAppEntry) -> Button {
        let widget = build_app_tile_widget(
            app,
            self.config.grid().tile_size(),
            self.config.grid().icon_size(),
            self.config.grid().icon_center_y_ratio(),
            self.config.grid().title_font_size(),
            self.config.grid().title_top_margin(),
            app.description(),
        );
        attach_launch_handler(&widget, app, &self.window);

        let add_state = self.quick_access.clone();
        let app_id = app.id().to_owned();
        let right_click = GestureClick::new();
        right_click.set_button(3);
        right_click.connect_pressed(move |_, _, _, _| {
            add_state.add_app(&app_id);
        });
        widget.add_controller(right_click);

        widget
    }
}

impl LauncherWindow {
    pub(super) fn populate_content_stack(
        &self,
        content_stack: &Stack,
        quick_access: &QuickAccessState,
        all_apps: &AllAppsPageState,
    ) {
        content_stack.add_css_class("launcher-content-stack");
        content_stack.set_hexpand(true);
        content_stack.set_vexpand(true);
        content_stack.set_transition_type(StackTransitionType::Crossfade);
        content_stack.add_named(&self.build_dashboard_page(quick_access), Some("dashboard"));
        content_stack.add_named(&self.build_all_apps_page(all_apps), Some("all-apps"));
    }

    fn build_dashboard_page(&self, quick_access: &QuickAccessState) -> gtk4::Box {
        quick_access.build_home_page()
    }

    fn build_all_apps_page(&self, all_apps: &AllAppsPageState) -> GtkBox {
        all_apps.build_page()
    }
}
