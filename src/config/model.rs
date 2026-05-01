use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct LauncherConfig {
    application_id: String,
    quick_access_path: PathBuf,
    theme: ThemeConfig,
    window: WindowConfig,
    character_video: CharacterVideoConfig,
    sidebar: SidebarConfig,
    top_panels: TopPanelsConfig,
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
    max_width: Option<i32>,
    width_ratio: f32,
    height_ratio: f32,
}

#[derive(Debug, Clone)]
pub struct CharacterVideoConfig {
    enabled: bool,
    path: Option<PathBuf>,
    outline_only: bool,
    height_ratio: f32,
    offset_x: i32,
    offset_y: i32,
}

#[derive(Debug, Clone)]
pub struct SidebarConfig {
    width: i32,
    scale: f32,
    button_scale: f32,
    button_bg_opacity: f32,
    colors: SidebarColors,
    spacing: i32,
    outer_margin: i32,
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
    colors: SidebarColors,
}

#[derive(Debug, Clone)]
pub(crate) struct SidebarSpacing {
    spacing: i32,
    outer_margin: i32,
    middle_offset: i32,
}

#[derive(Debug, Clone)]
pub struct TopPanelsConfig {
    height: i32,
    spacing: i32,
    top_margin: i32,
    outer_margin: i32,
    left_panels: Vec<TopPanelConfig>,
    right_panels: Vec<TopPanelConfig>,
}

#[derive(Debug, Clone)]
pub enum TopPanelConfig {
    Avatar(AvatarPanelConfig),
    Profile(ProfilePanelConfig),
    Text(TextPanelConfig),
    Search(SearchPanelConfig),
}

#[derive(Debug, Clone)]
pub struct AvatarPanelConfig {
    image_path: Option<PathBuf>,
    label: String,
    size: i32,
}

#[derive(Debug, Clone)]
pub struct ProfilePanelConfig {
    background_path: Option<PathBuf>,
    avatar_path: Option<PathBuf>,
    avatar_label: String,
    avatar_size: i32,
    uid: Option<String>,
    action_text: Option<String>,
    title: String,
    subtitle: Option<String>,
    items: Vec<ProfileItemConfig>,
}

#[derive(Debug, Clone)]
pub enum ProfileItemConfig {
    Text(ProfileTextItemConfig),
    Progress(ProfileProgressItemConfig),
}

#[derive(Debug, Clone)]
pub struct ProfileTextItemConfig {
    label: String,
    value: String,
}

#[derive(Debug, Clone)]
pub struct ProfileProgressItemConfig {
    label: String,
    value: String,
    progress: f64,
}

#[derive(Debug, Clone)]
pub struct TextPanelConfig {
    variant: TextPanelVariant,
    title: Option<String>,
    body: Option<String>,
    badge: Option<String>,
    min_height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextPanelVariant {
    Hero,
    Card,
}

#[derive(Debug, Clone)]
pub struct SearchPanelConfig {
    title: Option<String>,
    placeholder: String,
    min_height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SidebarColors {
    background_start: CssColor,
    background_end: CssColor,
    button_fg: CssColor,
    button_bg: CssColor,
    button_border: CssColor,
    button_active_bg: CssColor,
    button_active_border: CssColor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CssColor {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

#[derive(Debug, Clone)]
pub struct GridSectionConfig {
    spacing: i32,
    tile_spacing: i32,
    columns: i32,
    tile_size: i32,
    side_margin: i32,
    icon_size: i32,
    icon_center_y_ratio: f32,
    title_font_size: i32,
    title_top_margin: i32,
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
        character_video: CharacterVideoConfig,
        sidebar: SidebarConfig,
        top_panels: TopPanelsConfig,
        grid: GridSectionConfig,
    ) -> Self {
        Self {
            application_id,
            quick_access_path,
            theme,
            window,
            character_video,
            sidebar,
            top_panels,
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

    pub fn character_video(&self) -> &CharacterVideoConfig {
        &self.character_video
    }

    pub(crate) fn character_video_mut(&mut self) -> &mut CharacterVideoConfig {
        &mut self.character_video
    }

    pub fn top_panels(&self) -> &TopPanelsConfig {
        &self.top_panels
    }

    pub(crate) fn top_panels_mut(&mut self) -> &mut TopPanelsConfig {
        &mut self.top_panels
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
        max_width: Option<i32>,
        width_ratio: f32,
        height_ratio: f32,
    ) -> Self {
        Self {
            title,
            namespace,
            min_width,
            max_width,
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

    pub fn max_width(&self) -> Option<i32> {
        self.max_width
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

    pub fn set_max_width(&mut self, max_width: Option<i32>) {
        self.max_width = max_width;
    }

    pub fn set_width_ratio(&mut self, width_ratio: f32) {
        self.width_ratio = width_ratio;
    }

    pub fn set_height_ratio(&mut self, height_ratio: f32) {
        self.height_ratio = height_ratio;
    }
}

impl CharacterVideoConfig {
    pub(crate) fn new(
        enabled: bool,
        path: Option<PathBuf>,
        outline_only: bool,
        height_ratio: f32,
        offset_x: i32,
        offset_y: i32,
    ) -> Self {
        Self {
            enabled,
            path,
            outline_only,
            height_ratio,
            offset_x,
            offset_y,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn outline_only(&self) -> bool {
        self.outline_only
    }

    pub fn height_ratio(&self) -> f32 {
        self.height_ratio
    }

    pub fn offset_x(&self) -> i32 {
        self.offset_x
    }

    pub fn offset_y(&self) -> i32 {
        self.offset_y
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_path(&mut self, path: Option<PathBuf>) {
        self.path = path;
    }

    pub fn set_outline_only(&mut self, outline_only: bool) {
        self.outline_only = outline_only;
    }

    pub fn set_height_ratio(&mut self, height_ratio: f32) {
        self.height_ratio = height_ratio;
    }

    pub fn set_offset_x(&mut self, offset_x: i32) {
        self.offset_x = offset_x;
    }

    pub fn set_offset_y(&mut self, offset_y: i32) {
        self.offset_y = offset_y;
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
            colors: metrics.sizing.colors,
            spacing: metrics.spacing.spacing,
            outer_margin: metrics.spacing.outer_margin,
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

    pub fn colors(&self) -> SidebarColors {
        self.colors
    }

    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn outer_margin(&self) -> i32 {
        self.outer_margin
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

    pub fn set_colors(&mut self, colors: SidebarColors) {
        self.colors = colors;
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
    pub(crate) fn new(
        width: i32,
        scale: f32,
        button_scale: f32,
        button_bg_opacity: f32,
        colors: SidebarColors,
    ) -> Self {
        Self {
            width,
            scale,
            button_scale,
            button_bg_opacity,
            colors,
        }
    }
}

impl SidebarSpacing {
    pub(crate) fn new(spacing: i32, outer_margin: i32, middle_offset: i32) -> Self {
        Self {
            spacing,
            outer_margin,
            middle_offset,
        }
    }
}

impl TopPanelsConfig {
    pub(crate) fn new(
        height: i32,
        spacing: i32,
        top_margin: i32,
        outer_margin: i32,
        left_panels: Vec<TopPanelConfig>,
        right_panels: Vec<TopPanelConfig>,
    ) -> Self {
        Self {
            height,
            spacing,
            top_margin,
            outer_margin,
            left_panels,
            right_panels,
        }
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn top_margin(&self) -> i32 {
        self.top_margin
    }

    pub fn outer_margin(&self) -> i32 {
        self.outer_margin
    }

    pub fn left_panels(&self) -> &[TopPanelConfig] {
        &self.left_panels
    }

    pub fn right_panels(&self) -> &[TopPanelConfig] {
        &self.right_panels
    }

    pub fn set_height(&mut self, height: i32) {
        self.height = height;
    }

    pub fn set_spacing(&mut self, spacing: i32) {
        self.spacing = spacing;
    }

    pub fn set_top_margin(&mut self, top_margin: i32) {
        self.top_margin = top_margin;
    }

    pub fn set_outer_margin(&mut self, outer_margin: i32) {
        self.outer_margin = outer_margin;
    }

    pub fn set_left_panels(&mut self, left_panels: Vec<TopPanelConfig>) {
        self.left_panels = left_panels;
    }

    pub fn set_right_panels(&mut self, right_panels: Vec<TopPanelConfig>) {
        self.right_panels = right_panels;
    }
}

impl AvatarPanelConfig {
    pub(crate) fn new(image_path: Option<PathBuf>, label: String, size: i32) -> Self {
        Self {
            image_path,
            label,
            size,
        }
    }

    pub fn image_path(&self) -> Option<&Path> {
        self.image_path.as_deref()
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn size(&self) -> i32 {
        self.size
    }
}

impl ProfilePanelConfig {
    pub(crate) fn new(
        background_path: Option<PathBuf>,
        avatar_path: Option<PathBuf>,
        avatar_label: String,
        avatar_size: i32,
        uid: Option<String>,
        action_text: Option<String>,
        title: String,
        subtitle: Option<String>,
        items: Vec<ProfileItemConfig>,
    ) -> Self {
        Self {
            background_path,
            avatar_path,
            avatar_label,
            avatar_size,
            uid,
            action_text,
            title,
            subtitle,
            items,
        }
    }

    pub fn background_path(&self) -> Option<&Path> {
        self.background_path.as_deref()
    }

    pub fn avatar_path(&self) -> Option<&Path> {
        self.avatar_path.as_deref()
    }

    pub fn avatar_label(&self) -> &str {
        &self.avatar_label
    }

    pub fn avatar_size(&self) -> i32 {
        self.avatar_size
    }

    pub fn uid(&self) -> Option<&str> {
        self.uid.as_deref()
    }

    pub fn action_text(&self) -> Option<&str> {
        self.action_text.as_deref()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn subtitle(&self) -> Option<&str> {
        self.subtitle.as_deref()
    }

    pub fn items(&self) -> &[ProfileItemConfig] {
        &self.items
    }
}

impl ProfileTextItemConfig {
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

impl ProfileProgressItemConfig {
    pub(crate) fn new(label: String, value: String, progress: f64) -> Self {
        Self {
            label,
            value,
            progress,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn progress(&self) -> f64 {
        self.progress
    }
}

impl TextPanelConfig {
    pub(crate) fn new(
        variant: TextPanelVariant,
        title: Option<String>,
        body: Option<String>,
        badge: Option<String>,
        min_height: i32,
    ) -> Self {
        Self {
            variant,
            title,
            body,
            badge,
            min_height,
        }
    }

    pub fn variant(&self) -> TextPanelVariant {
        self.variant
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn badge(&self) -> Option<&str> {
        self.badge.as_deref()
    }

    pub fn min_height(&self) -> i32 {
        self.min_height
    }
}

impl SearchPanelConfig {
    pub(crate) fn new(title: Option<String>, placeholder: String, min_height: i32) -> Self {
        Self {
            title,
            placeholder,
            min_height,
        }
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn placeholder(&self) -> &str {
        &self.placeholder
    }

    pub fn min_height(&self) -> i32 {
        self.min_height
    }
}

impl SidebarColors {
    pub const fn new(
        background_start: CssColor,
        background_end: CssColor,
        button_fg: CssColor,
        button_bg: CssColor,
        button_border: CssColor,
        button_active_bg: CssColor,
        button_active_border: CssColor,
    ) -> Self {
        Self {
            background_start,
            background_end,
            button_fg,
            button_bg,
            button_border,
            button_active_bg,
            button_active_border,
        }
    }

    pub const fn background_start(self) -> CssColor {
        self.background_start
    }

    pub const fn background_end(self) -> CssColor {
        self.background_end
    }

    pub const fn button_fg(self) -> CssColor {
        self.button_fg
    }

    pub const fn button_bg(self) -> CssColor {
        self.button_bg
    }

    pub const fn button_border(self) -> CssColor {
        self.button_border
    }

    pub const fn button_active_bg(self) -> CssColor {
        self.button_active_bg
    }

    pub const fn button_active_border(self) -> CssColor {
        self.button_active_border
    }

    pub fn set_background_start(&mut self, color: CssColor) {
        self.background_start = color;
    }

    pub fn set_background_end(&mut self, color: CssColor) {
        self.background_end = color;
    }

    pub fn set_button_fg(&mut self, color: CssColor) {
        self.button_fg = color;
    }

    pub fn set_button_bg(&mut self, color: CssColor) {
        self.button_bg = color;
    }

    pub fn set_button_border(&mut self, color: CssColor) {
        self.button_border = color;
    }

    pub fn set_button_active_bg(&mut self, color: CssColor) {
        self.button_active_bg = color;
    }

    pub fn set_button_active_border(&mut self, color: CssColor) {
        self.button_active_border = color;
    }
}

impl CssColor {
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub const fn red(self) -> u8 {
        self.red
    }

    pub const fn green(self) -> u8 {
        self.green
    }

    pub const fn blue(self) -> u8 {
        self.blue
    }

    pub fn alpha(self) -> f32 {
        f32::from(self.alpha) / 255.0
    }
}

impl GridSectionConfig {
    pub(crate) fn new(
        spacing: i32,
        tile_spacing: i32,
        columns: i32,
        tile_size: i32,
        side_margin: i32,
        icon_size: i32,
        icon_center_y_ratio: f32,
        title_font_size: i32,
        title_top_margin: i32,
        buttons: Vec<ButtonConfig>,
    ) -> Self {
        Self {
            spacing,
            tile_spacing,
            columns,
            tile_size,
            side_margin,
            icon_size,
            icon_center_y_ratio,
            title_font_size,
            title_top_margin,
            buttons,
        }
    }

    pub fn spacing(&self) -> i32 {
        self.spacing
    }

    pub fn columns(&self) -> i32 {
        self.columns
    }

    pub fn tile_spacing(&self) -> i32 {
        self.tile_spacing
    }

    pub fn tile_size(&self) -> i32 {
        self.tile_size
    }

    pub fn side_margin(&self) -> i32 {
        self.side_margin
    }

    pub fn icon_size(&self) -> i32 {
        self.icon_size
    }

    pub fn icon_center_y_ratio(&self) -> f32 {
        self.icon_center_y_ratio
    }

    pub fn title_font_size(&self) -> i32 {
        self.title_font_size
    }

    pub fn title_top_margin(&self) -> i32 {
        self.title_top_margin
    }

    pub fn set_tile_size(&mut self, tile_size: i32) {
        self.tile_size = tile_size;
    }

    pub fn set_tile_spacing(&mut self, tile_spacing: i32) {
        self.tile_spacing = tile_spacing;
    }

    pub fn set_side_margin(&mut self, side_margin: i32) {
        self.side_margin = side_margin;
    }

    pub fn set_icon_size(&mut self, icon_size: i32) {
        self.icon_size = icon_size;
    }

    pub fn set_icon_center_y_ratio(&mut self, icon_center_y_ratio: f32) {
        self.icon_center_y_ratio = icon_center_y_ratio;
    }

    pub fn set_title_font_size(&mut self, title_font_size: i32) {
        self.title_font_size = title_font_size;
    }

    pub fn set_title_top_margin(&mut self, title_top_margin: i32) {
        self.title_top_margin = title_top_margin;
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
