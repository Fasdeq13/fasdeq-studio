use egui::text::LayoutJob;
use egui::{Color32, FontId, TextFormat};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Theme, ThemeSet};
use syntect::parsing::{SyntaxReference, SyntaxSet};

pub struct HighlightEngine {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl HighlightEngine {
    pub fn new() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn syntax_for_extension(&self, extension: &str) -> &SyntaxReference {
        let mapped = match extension {
            "c" | "h" => "c",
            "cpp" | "cc" | "cxx" | "hpp" | "hh" => "cpp",
            "rs" => "rs",
            "asm" | "s" | "nasm" => "asm",
            "toml" => "toml",
            "json" => "json",
            "md" => "md",
            "sh" => "sh",
            "py" => "py",
            "js" => "js",
            "ts" => "ts",
            other => other,
        };
        self.syntax_set
            .find_syntax_by_extension(mapped)
            .or_else(|| self.syntax_set.find_syntax_by_name(extension))
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text())
    }

    pub fn theme(&self, name: &str) -> &Theme {
        self.theme_set
            .themes
            .get(name)
            .unwrap_or_else(|| self.theme_set.themes.get("base16-eighties.dark").unwrap())
    }

    pub fn highlight(
        &self,
        text: &str,
        extension: &str,
        theme_name: &str,
        font_size: f32,
    ) -> LayoutJob {
        let syntax = self.syntax_for_extension(extension);
        let theme = self.theme(theme_name);
        let mut highlighter = HighlightLines::new(syntax, theme);

        let mut job = LayoutJob::default();
        for line in text.split_inclusive('\n') {
            let ranges = highlighter
                .highlight_line(line, &self.syntax_set)
                .unwrap_or_default();
            for (style, fragment) in ranges {
                let color = Color32::from_rgb(
                    style.foreground.r,
                    style.foreground.g,
                    style.foreground.b,
                );
                job.append(
                    fragment,
                    0.0,
                    TextFormat {
                        font_id: FontId::monospace(font_size),
                        color,
                        ..Default::default()
                    },
                );
            }
        }
        job
    }
}

impl Default for HighlightEngine {
    fn default() -> Self {
        Self::new()
    }
}
