pub use egui_phosphor::regular as icons;

pub const FOLDER: &str = icons::FOLDER;
pub const FOLDER_OPEN: &str = icons::FOLDER_OPEN;
pub const FOLDER_PLUS: &str = icons::FOLDER_PLUS;
pub const FILE: &str = icons::FILE;
pub const FILE_C: &str = icons::FILE_C;
pub const FILE_CPP: &str = icons::FILE_CPP;
pub const FILE_RS: &str = icons::FILE_RS;
pub const FILE_CODE: &str = icons::FILE_CODE;
pub const FILE_TEXT: &str = icons::FILE_TEXT;
pub const GEAR: &str = icons::GEAR;
pub const GEAR_SIX: &str = icons::GEAR_SIX;
pub const GLOBE: &str = icons::GLOBE;
pub const SIGN_OUT: &str = icons::SIGN_OUT;
pub const ARROW_CLOCKWISE: &str = icons::ARROW_CLOCKWISE;
pub const ARROW_COUNTER_CLOCKWISE: &str = icons::ARROW_COUNTER_CLOCKWISE;
pub const ARROW_LEFT: &str = icons::ARROW_LEFT;
pub const PLAY: &str = icons::PLAY;
pub const FLOPPY_DISK: &str = icons::FLOPPY_DISK;
pub const MAGNIFYING_GLASS: &str = icons::MAGNIFYING_GLASS;
pub const BOOK_OPEN_TEXT: &str = icons::BOOK_OPEN_TEXT;
pub const WRENCH: &str = icons::WRENCH;
pub const PUZZLE_PIECE: &str = icons::PUZZLE_PIECE;
pub const X: &str = icons::X;
pub const X_CIRCLE: &str = icons::X_CIRCLE;
pub const CHECK_CIRCLE: &str = icons::CHECK_CIRCLE;
pub const WARNING: &str = icons::WARNING;
pub const WARNING_CIRCLE: &str = icons::WARNING_CIRCLE;
pub const INFO: &str = icons::INFO;
pub const PLUS_CIRCLE: &str = icons::PLUS_CIRCLE;
pub const MOON: &str = icons::MOON;
pub const SUN: &str = icons::SUN;
pub const CARET_RIGHT: &str = icons::CARET_RIGHT;
pub const CARET_DOWN: &str = icons::CARET_DOWN;
pub const CHAT_CIRCLE: &str = icons::CHAT_CIRCLE;
pub const DOT: &str = icons::CIRCLE;
pub const BINARY: &str = icons::BINARY;
pub const HASH: &str = icons::HASH;
pub const CUBE: &str = icons::CUBE;
pub const TERMINAL: &str = icons::TERMINAL_WINDOW;
pub const NOTE_PENCIL: &str = icons::NOTE_PENCIL;
pub const DOWNLOAD: &str = icons::DOWNLOAD_SIMPLE;
pub const GIT_BRANCH: &str = icons::GIT_BRANCH;
pub const UPLOAD_SIMPLE: &str = icons::UPLOAD_SIMPLE;
pub const TRASH: &str = icons::TRASH;
pub const FOLDER_NOTCH_OPEN: &str = icons::FOLDER_NOTCH_OPEN;

pub fn extension_icon(extension: &str) -> &'static str {
    match extension {
        "rs" => FILE_RS,
        "c" | "h" => FILE_C,
        "cpp" | "cc" | "cxx" | "hpp" | "hh" => FILE_CPP,
        "asm" | "s" => FILE_CODE,
        "toml" | "json" | "yaml" | "yml" => GEAR_SIX,
        "md" => FILE_TEXT,
        _ => FILE,
    }
}
