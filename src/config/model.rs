use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct LauncherConfig {
    application_id: String,
    quick_access_path: PathBuf,
    theme: ThemeConfig,
    window: WindowConfig,
    sidebar: SidebarConfig,
    header: HeaderBannerConfig,
    grid: GridSectionConfig,
}

#[derive(Debug, Clone)]
pub struct ThemeConfig {
    font_family: Option<String>,
    font_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    title: String,
    namespace: String,
    min_width: i32,
    width_ratio: f32,
    height_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct SidebarConfig {
    width: i32,
    scale: f32,
    button_scale: f32,
    button_bg_opacity: f32,
    spacing: i32,
    outer_margin: i32,
    inner_margin: i32,
    middle_offset: i32,
    top_button: ButtonConfig,
    buttons: Vec<ButtonConfig>,
    bottom_button: ButtonConfig,
}

#[derive(Debug, Clone)]
pub(crate) struct SidebarMetrics {
    sizing: SidebarSizing,
    spacing: SidebarSpacing,
}

#[derive(Debug, Clone)]
pub(crate) struct SidebarSizing {
    width: i32,
    scale: f32,
    button_scale: f32,
    button_bg_opacity: f32,
}

#[derive(Debug, Clone)]
pub(crate) struct SidebarSpacing {
    spacing: i32,
    outer_margin: i32,
    inner_margin: i32,
    middle_offset: i32,
}

#[derive(Debug, Clone)]
pub struct HeaderBannerConfig {
    height: i32,
    spacing: i32,
    outer_margin: i32,
    title: String,
    subtitle: String,
    stats: Vec<HeaderStatConfig>,
    action_buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone)]
pub struct HeaderStatConfig {
    label: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct GridSectionConfig {
    spacing: i32,
    columns: i32,
    tile_size: i32,
    buttons: Vec<ButtonConfig>,
}

#[derive(Debug, Clone)]
pub struct ButtonConfig {
    id: String,
    label: String,
    icon_name: Option<String>,
    icon_path: Option<PathBuf>,
    action: MenuAction,
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    None,
    CloseMenu,
    OpenSection(String),
}

impl LauncherConfig {
    pub(crate) fn new(
        application_id: String,
        quick_access_path: PathBuf,
        theme: ThemeConfig,
        window: WindowConfig,
        sidebar: SidebarConfig,
        header: HeaderBannerConfig,
        grid: GridSectionConfig,
    ) -> Self {
        Self {
            application_id,
            quick_access_path,
            theme,
            window,
            sidebar,
            header,
            grid,
        }
    }

    pub fn application_id(&self) -> &str {
        &self.application_id
    }

    pub fn quick_access_path(&self) -> &Path {
        &self.quick_access_path
    }

    pub fn set_quick_access_path(&mut self, quick_access_path: PathBuf) {
        self.quick_access_path = quick_access_path;
    }

    pub fn theme(&self) -> &ThemeConfig {
        &self.theme
    }

    pub(crate) fn theme_mut(&mut self) -> &mut ThemeConfig {
        &mut self.theme
    }

    pub fn window(&self) -> &WindowConfig {
        &self.window
    }

    pub(crate) fn window_mut(&mut self) -> &mut WindowConfig {
        &mut self.window
    }

    pub fn sidebar(&self) -> &SidebarConfig {
        &self.sidebar
    }

    pub(crate) fn sidebar_mut(&mut self) -> &mut SidebarConfig {
        &mut self.sidebar
    }

    pub fn header(&self) -> &HeaderBannerConfig {
        &self.header
    }

    pub fn grid(&self) -> &GridSectionConfig {
        &self.grid
    }

    pub(crate) fn grid_mut(&mut self) -> &mut GridSectionConfig {
        &mut self.grid
    }
}

impl ThemeConfig {
    pub(crate) fn new(font_family: Option<String>, font_path: Option<PathBuf>) -> Self {
        Self {
            font_family,
            font_path,
        }
    }

    pub fn font_family(&self) -> Option<&str> {
        self.font_family.as_deref()
    }

    pub fn font_path(&self) -> Option<&Path> {
        self.font_path.as_deref()
    }

    pub fn set_font_family(&mut self, font_family: Option<String>) {
        self.font_family = font_family;
    }

    pub fn set_font_path(&mut self, font_path: Option<PathBuf>) {
        self.font_path = font_path;
    }
}

impl WindowConfig {
    pub(crate) fn new(
        title: String,
        namespace: String,
        min_width: i32,
        width_ratio: f32,
        height_ratio: f32,
    ) -> Self {
        Self {
            title,
            namespace,
            min_width,
            width_ratio,
            height_ratio,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn min_width(&self) -> i32 {
        self.min_width
    }

    pub fn width_ratio(&self) -> f32 {
        self.width_ratio
    }

    pub fn height_ratio(&self) -> f32 {
        self.height_ratio
    }

    pub fn set_min_width(&mut self, min_width: i32) {
        self.min_width = min_width;
    }

    pub fn set_width_ratio(&mut self, width_ratio: f32) {
        self.width_ratio = width_ratio;
    }

    pub fn set_height_ratio(&mut self, height_ratio: f32) {
        self.height_ratio = height_ratio;
    }
}

impl SidebarConfig {
    pub(crate) fn new(
        metrics: SidebarMetrics,
        top_button: ButtonConfig,
        buttons: Vec<ButtonConfig>,
        bottom_button: ButtonConfig,
    ) -> Self {
        Self {
            width: metrics.sizing.width,
            scale: metrics.sizing.scale,
            button_scale: metrics.sizing.button_scale,
            button_bg_opacity: metrics.sizing.button_bg_opacity,
            spacing: metrics.spacing.spacing,
            outer_margin: metrics.spacing.outer_margin,
            inner_margin: metrics.spacing.inner_margin,
            middle_offset: metrics.spacing.middle_offset,
            top_button,
            buttons,
            bottom_button,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn button_scale(&self) -> f32 {
        self.button_scale
    }

    pub fn button_bg_opacity(&self) -> f32 {
        self.button_bg_opacity
    }

    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn outer_margin(&self) -> i32 {
        self.outer_margin
    }

    pub fn inner_margin(&self) -> i32 {
        self.inner_margin
    }

    pub fn middle_offset(&self) -> i32 {
        self.middle_offset
    }

    pub fn top_button(&self) -> &ButtonConfig {
        &self.top_button
    }

    pub fn buttons(&self) -> &[ButtonConfig] {
        &self.buttons
    }

    pub fn bottom_button(&self) -> &ButtonConfig {
        &self.bottom_button
    }

    pub fn set_width(&mut self, width: i32) {
        self.width = width;
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_button_scale(&mut self, button_scale: f32) {
        self.button_scale = button_scale;
    }

    pub fn set_button_bg_opacity(&mut self, button_bg_opacity: f32) {
        self.button_bg_opacity = button_bg_opacity;
    }

    pub fn set_buttons(&mut self, buttons: Vec<ButtonConfig>) {
        self.buttons = buttons;
    }

    pub(crate) fn top_button_mut(&mut self) -> &mut ButtonConfig {
        &mut self.top_button
    }

    pub(crate) fn bottom_button_mut(&mut self) -> &mut ButtonConfig {
        &mut self.bottom_button
    }
}

impl SidebarMetrics {
    pub(crate) fn new(sizing: SidebarSizing, spacing: SidebarSpacing) -> Self {
        Self { sizing, spacing }
    }
}

impl SidebarSizing {
    pub(crate) fn new(width: i32, scale: f32, button_scale: f32, button_bg_opacity: f32) -> Self {
        Self {
            width,
            scale,
            button_scale,
            button_bg_opacity,
        }
    }
}

impl SidebarSpacing {
    pub(crate) fn new(
        spacing: i32,
        outer_margin: i32,
        inner_margin: i32,
        middle_offset: i32,
    ) -> Self {
        Self {
            spacing,
            outer_margin,
            inner_margin,
            middle_offset,
        }
    }
}

impl HeaderBannerConfig {
    pub(crate) fn new(
        height: i32,
        spacing: i32,
        outer_margin: i32,
        title: String,
        subtitle: String,
        stats: Vec<HeaderStatConfig>,
        action_buttons: Vec<ButtonConfig>,
    ) -> Self {
        Self {
            height,
            spacing,
            outer_margin,
            title,
            subtitle,
            stats,
            action_buttons,
        }
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn outer_margin(&self) -> i32 {
        self.outer_margin
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn subtitle(&self) -> &str {
        &self.subtitle
    }

    pub fn stats(&self) -> &[HeaderStatConfig] {
        &self.stats
    }

    pub fn action_buttons(&self) -> &[ButtonConfig] {
        &self.action_buttons
    }
}

impl HeaderStatConfig {
    pub(crate) fn new(label: String, value: String) -> Self {
        Self { label, value }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

impl GridSectionConfig {
    pub(crate) fn new(
        spacing: i32,
        columns: i32,
        tile_size: i32,
        buttons: Vec<ButtonConfig>,
    ) -> Self {
        Self {
            spacing,
            columns,
            tile_size,
            buttons,
        }
    }

    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn columns(&self) -> i32 {
        self.columns
    }

    pub fn tile_size(&self) -> i32 {
        self.tile_size
    }

    pub fn set_tile_size(&mut self, tile_size: i32) {
        self.tile_size = tile_size;
    }

    pub fn set_buttons(&mut self, buttons: Vec<ButtonConfig>) {
        self.buttons = buttons;
    }
}

impl ButtonConfig {
    pub(crate) fn new(
        id: String,
        label: String,
        icon_name: Option<String>,
        icon_path: Option<PathBuf>,
        action: MenuAction,
    ) -> Self {
        Self {
            id,
            label,
            icon_name,
            icon_path,
            action,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn icon_name(&self) -> Option<&str> {
        self.icon_name.as_deref()
    }

    pub fn icon_path(&self) -> Option<&Path> {
        self.icon_path.as_deref()
    }

    pub fn action(&self) -> &MenuAction {
        &self.action
    }

    pub(crate) fn set_id(&mut self, id: String) {
        self.id = id;
    }

    pub(crate) fn set_label(&mut self, label: String) {
        self.label = label;
    }

    pub(crate) fn set_icon_name(&mut self, icon_name: Option<String>) {
        self.icon_name = icon_name;
    }

    pub(crate) fn set_icon_path(&mut self, icon_path: Option<PathBuf>) {
        self.icon_path = icon_path;
    }
}
