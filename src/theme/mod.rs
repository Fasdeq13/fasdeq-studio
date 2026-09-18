use egui::{Color32, Rounding, Stroke, Vec2, Shadow};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThemeColors {
    pub background: [u8; 3],
    pub panel: [u8; 3],
    pub panel_alt: [u8; 3],
    pub accent: [u8; 3],
    pub accent_hover: [u8; 3],
    pub text_primary: [u8; 3],
    pub text_secondary: [u8; 3],
    pub text_muted: [u8; 3],
    pub border: [u8; 3],
    pub error: [u8; 3],
    pub warning: [u8; 3],
    pub success: [u8; 3],
    pub selection: [u8; 3],
    pub editor_bg: [u8; 3],
    pub editor_gutter: [u8; 3],
    pub editor_cursor_line: [u8; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FasdeqTheme {
    pub name: String,
    pub is_dark: bool,
    pub colors: ThemeColors,
    pub rounding: f32,
    pub syntax_theme_name: String,
}

impl FasdeqTheme {
    pub fn dark_default() -> Self {
        Self {
            name: "Fasdeq Dark".into(),
            is_dark: true,
            colors: ThemeColors {
                background: [18, 19, 24],
                panel: [24, 25, 32],
                panel_alt: [30, 32, 40],
                accent: [122, 92, 255],
                accent_hover: [148, 122, 255],
                text_primary: [230, 231, 238],
                text_secondary: [170, 173, 190],
                text_muted: [110, 113, 130],
                border: [42, 44, 56],
                error: [242, 95, 92],
                warning: [240, 180, 90],
                success: [96, 210, 150],
                selection: [60, 50, 110],
                editor_bg: [16, 17, 22],
                editor_gutter: [21, 22, 28],
                editor_cursor_line: [26, 28, 36],
            },
            rounding: 8.0,
            syntax_theme_name: "base16-eighties.dark".into(),
        }
    }

    pub fn light_default() -> Self {
        Self {
            name: "Fasdeq Light".into(),
            is_dark: false,
            colors: ThemeColors {
                background: [246, 247, 250],
                panel: [255, 255, 255],
                panel_alt: [238, 240, 245],
                accent: [102, 74, 226],
                accent_hover: [128, 100, 240],
                text_primary: [30, 32, 40],
                text_secondary: [70, 73, 90],
                text_muted: [140, 143, 160],
                border: [222, 224, 232],
                error: [214, 60, 58],
                warning: [200, 140, 30],
                success: [40, 160, 100],
                selection: [220, 210, 250],
                editor_bg: [252, 252, 254],
                editor_gutter: [244, 245, 248],
                editor_cursor_line: [238, 238, 245],
            },
            rounding: 8.0,
            syntax_theme_name: "InspiredGitHub".into(),
        }
    }

    pub fn midnight_purple() -> Self {
        Self {
            name: "Midnight Purple".into(),
            is_dark: true,
            colors: ThemeColors {
                background: [12, 10, 22],
                panel: [20, 16, 34],
                panel_alt: [28, 22, 46],
                accent: [186, 104, 255],
                accent_hover: [206, 140, 255],
                text_primary: [230, 224, 245],
                text_secondary: [180, 170, 210],
                text_muted: [120, 112, 150],
                border: [48, 38, 72],
                error: [255, 90, 120],
                warning: [255, 190, 90],
                success: [100, 230, 180],
                selection: [70, 40, 110],
                editor_bg: [10, 8, 18],
                editor_gutter: [16, 13, 28],
                editor_cursor_line: [24, 18, 40],
            },
            rounding: 10.0,
            syntax_theme_name: "base16-ocean.dark".into(),
        }
    }

    pub fn solar_ember() -> Self {
        Self {
            name: "Solar Ember".into(),
            is_dark: true,
            colors: ThemeColors {
                background: [24, 18, 16],
                panel: [32, 24, 22],
                panel_alt: [40, 30, 27],
                accent: [255, 130, 60],
                accent_hover: [255, 160, 100],
                text_primary: [240, 228, 220],
                text_secondary: [200, 180, 168],
                text_muted: [140, 120, 108],
                border: [60, 44, 38],
                error: [255, 90, 80],
                warning: [255, 200, 80],
                success: [140, 220, 110],
                selection: [90, 55, 30],
                editor_bg: [20, 15, 13],
                editor_gutter: [26, 20, 18],
                editor_cursor_line: [34, 26, 23],
            },
            rounding: 6.0,
            syntax_theme_name: "base16-eighties.dark".into(),
        }
    }

    pub fn glacier_light() -> Self {
        Self {
            name: "Glacier".into(),
            is_dark: false,
            colors: ThemeColors {
                background: [240, 246, 250],
                panel: [255, 255, 255],
                panel_alt: [226, 238, 246],
                accent: [30, 140, 200],
                accent_hover: [50, 165, 225],
                text_primary: [20, 34, 42],
                text_secondary: [60, 80, 92],
                text_muted: [130, 150, 162],
                border: [206, 222, 232],
                error: [210, 65, 65],
                warning: [200, 145, 30],
                success: [35, 155, 110],
                selection: [190, 222, 240],
                editor_bg: [250, 253, 255],
                editor_gutter: [236, 244, 249],
                editor_cursor_line: [228, 240, 247],
            },
            rounding: 8.0,
            syntax_theme_name: "InspiredGitHub".into(),
        }
    }

    pub fn built_in_themes() -> Vec<FasdeqTheme> {
        vec![
            Self::dark_default(),
            Self::light_default(),
            Self::midnight_purple(),
            Self::solar_ember(),
            Self::glacier_light(),
        ]
    }

    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        if path.extension().and_then(|e| e.to_str()) == Some("css") {
            Self::from_css(&content)
        } else {
            let theme: FasdeqTheme = toml::from_str(&content)?;
            Ok(theme)
        }
    }

    fn from_css(css: &str) -> anyhow::Result<Self> {
        let mut theme = Self::dark_default();
        theme.name = "Custom CSS Theme".into();
        for line in css.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("--fasdeq-") {
                if let Some((key, value)) = rest.split_once(':') {
                    let value = value.trim().trim_end_matches(';').trim();
                    if let Some(color) = parse_css_color(value) {
                        apply_color_to_field(&mut theme.colors, key.trim(), color);
                    }
                }
            }
        }
        Ok(theme)
    }

    pub fn save_to_file(&self, path: &Path) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = if self.is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        let c = &self.colors;
        visuals.override_text_color = Some(rgb(c.text_primary));
        visuals.window_fill = rgb(c.panel);
        visuals.panel_fill = rgb(c.background);
        visuals.faint_bg_color = rgb(c.panel_alt);
        visuals.extreme_bg_color = rgb(c.editor_bg);
        visuals.code_bg_color = rgb(c.editor_bg);
        visuals.window_stroke = Stroke::new(1.0, rgb(c.border));
        visuals.widgets.noninteractive.bg_fill = rgb(c.panel);
        visuals.widgets.noninteractive.weak_bg_fill = rgb(c.panel_alt);
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, rgb(c.text_secondary));
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, rgb(c.border));
        visuals.widgets.inactive.bg_fill = rgb(c.panel_alt);
        visuals.widgets.inactive.weak_bg_fill = rgb(c.panel_alt);
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, rgb(c.text_secondary));
        visuals.widgets.hovered.bg_fill = rgb(c.accent_hover);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.2, rgb(c.text_primary));
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, rgb(c.accent));
        visuals.widgets.active.bg_fill = rgb(c.accent);
        visuals.widgets.active.fg_stroke = Stroke::new(1.2, Color32::WHITE);
        visuals.widgets.open.bg_fill = rgb(c.panel_alt);
        visuals.selection.bg_fill = rgb(c.selection);
        visuals.selection.stroke = Stroke::new(1.0, rgb(c.accent));
        visuals.hyperlink_color = rgb(c.accent);
        visuals.error_fg_color = rgb(c.error);
        visuals.warn_fg_color = rgb(c.warning);
        visuals.window_rounding = Rounding::same(self.rounding);
        visuals.menu_rounding = Rounding::same(self.rounding * 0.75);
        visuals.widgets.noninteractive.rounding = Rounding::same(self.rounding * 0.6);
        visuals.widgets.inactive.rounding = Rounding::same(self.rounding * 0.6);
        visuals.widgets.hovered.rounding = Rounding::same(self.rounding * 0.6);
        visuals.widgets.active.rounding = Rounding::same(self.rounding * 0.6);
        visuals.window_shadow = Shadow {
            offset: Vec2::new(0.0, 6.0),
            blur: 24.0,
            spread: 0.0,
            color: Color32::from_black_alpha(90),
        };
        visuals.popup_shadow = visuals.window_shadow;

        let mut style = (*ctx.style()).clone();
        style.visuals = visuals;
        style.spacing.item_spacing = Vec2::new(10.0, 8.0);
        style.spacing.button_padding = Vec2::new(14.0, 8.0);
        style.spacing.window_margin = egui::Margin::same(16.0);
        ctx.set_style(style);
    }
}

fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

fn parse_css_color(value: &str) -> Option<[u8; 3]> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        let hex = hex.trim();
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some([r, g, b]);
        }
        if hex.len() == 3 {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            return Some([r, g, b]);
        }
    }
    if let Some(inner) = value.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
        let parts: Vec<u8> = inner
            .split(',')
            .filter_map(|p| p.trim().parse::<u8>().ok())
            .collect();
        if parts.len() == 3 {
            return Some([parts[0], parts[1], parts[2]]);
        }
    }
    None
}

fn apply_color_to_field(colors: &mut ThemeColors, key: &str, color: [u8; 3]) {
    match key {
        "background" => colors.background = color,
        "panel" => colors.panel = color,
        "panel-alt" => colors.panel_alt = color,
        "accent" => colors.accent = color,
        "accent-hover" => colors.accent_hover = color,
        "text-primary" => colors.text_primary = color,
        "text-secondary" => colors.text_secondary = color,
        "text-muted" => colors.text_muted = color,
        "border" => colors.border = color,
        "error" => colors.error = color,
        "warning" => colors.warning = color,
        "success" => colors.success = color,
        "selection" => colors.selection = color,
        "editor-bg" => colors.editor_bg = color,
        "editor-gutter" => colors.editor_gutter = color,
        "editor-cursor-line" => colors.editor_cursor_line = color,
        _ => {}
    }
}
