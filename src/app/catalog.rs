use std::collections::HashMap;
use std::rc::Rc;

use gtk4::gio::AppInfo;
use gtk4::prelude::AppInfoExt;

#[derive(Clone)]
pub(crate) struct DesktopAppEntry {
    app_info: AppInfo,
    id: String,
    name: String,
    description: Option<String>,
}

#[derive(Clone)]
pub(crate) struct DesktopAppCatalog {
    ordered: Rc<Vec<DesktopAppEntry>>,
    by_id: Rc<HashMap<String, DesktopAppEntry>>,
}

impl DesktopAppCatalog {
    pub(crate) fn collect() -> Self {
        let mut apps: Vec<_> = AppInfo::all()
            .into_iter()
            .filter(|app| app.should_show())
            .filter_map(|app| {
                let name = app.display_name().to_string();
                if name.trim().is_empty() {
                    return None;
                }

                let id = app
                    .id()
                    .map(|id| id.to_string())
                    .filter(|id| !id.trim().is_empty())
                    .unwrap_or_else(|| app.executable().to_string_lossy().to_string());

                Some(DesktopAppEntry {
                    description: app.description().map(|description| description.to_string()),
                    app_info: app,
                    id,
                    name,
                })
            })
            .collect();

        apps.sort_by_cached_key(|app| app.name.to_lowercase());
        apps.dedup_by(|left, right| left.id == right.id || left.name == right.name);

        Self::new(apps)
    }

    fn new(apps: Vec<DesktopAppEntry>) -> Self {
        let by_id = apps
            .iter()
            .cloned()
            .map(|app| (app.id.clone(), app))
            .collect::<HashMap<_, _>>();

        Self {
            ordered: Rc::new(apps),
            by_id: Rc::new(by_id),
        }
    }

    pub(crate) fn ordered(&self) -> &[DesktopAppEntry] {
        self.ordered.as_ref().as_slice()
    }

    pub(crate) fn get(&self, app_id: &str) -> Option<&DesktopAppEntry> {
        self.by_id.get(app_id)
    }
}

impl DesktopAppEntry {
    pub(crate) fn app_info(&self) -> &AppInfo {
        &self.app_info
    }

    pub(crate) fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub(crate) fn id(&self) -> &str {
        &self.id
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
