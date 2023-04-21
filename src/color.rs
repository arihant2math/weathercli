pub const FORE_BLACK: &str = "\x1b[30m";
pub const FORE_RED: &str = "\x1b[31m";
pub const FORE_GREEN: &str = "\x1b[32m";
pub const FORE_YELLOW: &str = "\x1b[33m";
pub const FORE_BLUE: &str = "\x1b[34m";
pub const FORE_MAGENTA: &str = "\x1b[35m";
pub const FORE_CYAN: &str = "\x1b[36m";
pub const FORE_WHITE: &str = "\x1b[37m";
pub const FORE_RESET: &str = "\x1b[39m";
// These are fairly well supported, but not part of the standard.
pub const FORE_LIGHTBLACK: &str = "\x1b[90m";
pub const FORE_LIGHTRED: &str = "\x1b[91m";
pub const FORE_LIGHTGREEN: &str = "\x1b[92m";
pub const FORE_LIGHTYELLOW: &str = "\x1b[93m";
pub const FORE_LIGHTBLUE: &str = "\x1b[94m";
pub const FORE_LIGHTMAGENTA: &str = "\x1b[95m";
pub const FORE_LIGHTCYAN: &str = "\x1b[96m";
pub const FORE_LIGHTWHITE: &str = "\x1b[97m";

pub fn from_string(s: String) -> Option<String> {
    match &*s {
        "FORE_BLACK" => Some(FORE_BLACK.to_string()),
        "FORE_RED" => Some(FORE_RED.to_string()),
        "FORE_GREEN" => Some(FORE_GREEN.to_string()),
        "FORE_YELLOW" => Some(FORE_YELLOW.to_string()),
        "FORE_BLUE" => Some(FORE_BLUE.to_string()),
        "FORE_MAGENTA" => Some(FORE_MAGENTA.to_string()),
        "FORE_CYAN" => Some(FORE_CYAN.to_string()),
        "FORE_WHITE" => Some(FORE_WHITE.to_string()),
        "FORE_RESET" => Some(FORE_RESET.to_string()),
        "FORE_LIGHTBLACK" => Some(FORE_LIGHTBLACK.to_string()),
        "FORE_LIGHTRED" => Some(FORE_LIGHTRED.to_string()),
        "FORE_LIGHTGREEN" => Some(FORE_LIGHTGREEN.to_string()),
        "FORE_LIGHTYELLOW" => Some(FORE_LIGHTYELLOW.to_string()),
        "FORE_LIGHTBLUE" => Some(FORE_LIGHTBLUE.to_string()),
        "FORE_LIGHTMAGENTA" => Some(FORE_LIGHTMAGENTA.to_string()),
        "FORE_LIGHTCYAN" => Some(FORE_LIGHTCYAN.to_string()),
        "FORE_LIGHTWHITE" => Some(FORE_LIGHTWHITE.to_string()),
        _ => None,
    }
}
