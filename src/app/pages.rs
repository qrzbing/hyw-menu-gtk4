use gtk4::prelude::*;
use gtk4::{
    Align, Button, GestureClick, Grid, Orientation, ScrolledWindow, Stack, StackTransitionType,
};

use super::catalog::{DesktopAppCatalog, DesktopAppEntry};
use super::launcher::LauncherWindow;
use super::quick_access::QuickAccessState;
use super::tiles::{attach_launch_handler, build_app_tile_widget};

impl LauncherWindow {
    pub(super) fn populate_content_stack(
        &self,
        content_stack: &Stack,
        catalog: &DesktopAppCatalog,
        quick_access: &QuickAccessState,
    ) {
        content_stack.add_css_class("launcher-content-stack");
        content_stack.set_hexpand(true);
        content_stack.set_vexpand(true);
        content_stack.set_transition_type(StackTransitionType::Crossfade);
        content_stack.add_named(&self.build_dashboard_page(quick_access), Some("dashboard"));
        content_stack.add_named(
            &self.build_all_apps_page(catalog, quick_access),
            Some("all-apps"),
        );
    }

    fn build_dashboard_page(&self, quick_access: &QuickAccessState) -> gtk4::Box {
        quick_access.build_home_page()
    }

    fn build_all_apps_page(
        &self,
        catalog: &DesktopAppCatalog,
        quick_access: &QuickAccessState,
    ) -> gtk4::Box {
        let page = gtk4::Box::new(Orientation::Vertical, 12);
        page.add_css_class("launcher-page");
        page.add_css_class("launcher-all-apps-page");
        page.set_hexpand(true);
        page.set_vexpand(true);
        page.append(&self.build_all_apps_scroller(catalog, quick_access));
        page
    }

    fn build_all_apps_scroller(
        &self,
        catalog: &DesktopAppCatalog,
        quick_access: &QuickAccessState,
    ) -> ScrolledWindow {
        let scroller = ScrolledWindow::builder()
            .hexpand(true)
            .vexpand(true)
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .build();
        scroller.add_css_class("launcher-apps-scroller");

        let grid = Grid::new();
        let spacing = self.config().grid().spacing().max(0) as u32;
        grid.set_column_spacing(spacing);
        grid.set_row_spacing(spacing);
        grid.set_halign(Align::Start);
        grid.set_valign(Align::Start);
        grid.set_hexpand(false);
        grid.set_vexpand(false);
        grid.add_css_class("launcher-apps-grid");

        for (index, app) in catalog.ordered().iter().enumerate() {
            let column = (index as i32) % self.config().grid().columns();
            let row = (index as i32) / self.config().grid().columns();
            grid.attach(
                &self.build_all_apps_tile(app, quick_access),
                column,
                row,
                1,
                1,
            );
        }

        scroller.set_child(Some(&grid));
        scroller
    }

    fn build_all_apps_tile(
        &self,
        app: &DesktopAppEntry,
        quick_access: &QuickAccessState,
    ) -> Button {
        let widget =
            build_app_tile_widget(app, self.config().grid().tile_size(), app.description());
        attach_launch_handler(&widget, app, self.window());

        let add_state = quick_access.clone();
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
