pub const RESET: &str = "\x1b[0m";
pub const FORE_RESET: &str = "\x1b[39m";
pub const BACK_RESET: &str = "\x1b[49m";

pub const FORE_BLACK: &str = "\x1b[30m";
pub const FORE_RED: &str = "\x1b[31m";
pub const FORE_GREEN: &str = "\x1b[32m";
pub const FORE_YELLOW: &str = "\x1b[33m";
pub const FORE_BLUE: &str = "\x1b[34m";
pub const FORE_MAGENTA: &str = "\x1b[35m";
pub const FORE_CYAN: &str = "\x1b[36m";
pub const FORE_WHITE: &str = "\x1b[37m";

pub const FORE_LIGHTBLACK: &str = "\x1b[90m";
pub const FORE_LIGHTRED: &str = "\x1b[91m";
pub const FORE_LIGHTGREEN: &str = "\x1b[92m";
pub const FORE_LIGHTYELLOW: &str = "\x1b[93m";
pub const FORE_LIGHTBLUE: &str = "\x1b[94m";
pub const FORE_LIGHTMAGENTA: &str = "\x1b[95m";
pub const FORE_LIGHTCYAN: &str = "\x1b[96m";
pub const FORE_LIGHTWHITE: &str = "\x1b[97m";

pub const BACK_BLACK: &str = "\x1b[40m";
pub const BACK_RED: &str = "\x1b[41m";
pub const BACK_GREEN: &str = "\x1b[42m";
pub const BACK_YELLOW: &str = "\x1b[43m";
pub const BACK_BLUE: &str = "\x1b[44m";
pub const BACK_MAGENTA: &str = "\x1b[45m";
pub const BACK_CYAN: &str = "\x1b[46m";
pub const BACK_WHITE: &str = "\x1b[47m";

pub const BACK_LIGHTBLACK: &str = "\x1b[100m";
pub const BACK_LIGHTRED: &str = "\x1b[101m";
pub const BACK_LIGHTGREEN: &str = "\x1b[102m";
pub const BACK_LIGHTYELLOW: &str = "\x1b[103m";
pub const BACK_LIGHTBLUE: &str = "\x1b[104m";
pub const BACK_LIGHTMAGENTA: &str = "\x1b[105m";
pub const BACK_LIGHTCYAN: &str = "\x1b[106m";
pub const BACK_LIGHTWHITE: &str = "\x1b[107m";

pub fn rgb(red: u8, green: u8, blue: u8) -> String {
    // TODO: This is Foreground only
    format!("\x1b[38;2;{red};{green};{blue}m")
}

pub fn string_to_rgb(s: &str) -> Option<String> {
    let split: Vec<&str> = s.split(',').collect();
    if split.len() != 3 {
        return None;
    }
    Some(rgb(
        split.first()?.parse().ok()?,
        split.get(1)?.parse().ok()?,
        split.get(2)?.parse().ok()?,
    ))
}

pub fn from_string(s: &str) -> Option<String> {
    match s {
        "RESET" => Some(RESET.to_string()),
        "FORE_BLACK" => Some(FORE_BLACK.to_string()),
        "FORE_RED" => Some(FORE_RED.to_string()),
        "FORE_GREEN" => Some(FORE_GREEN.to_string()),
        "FORE_YELLOW" => Some(FORE_YELLOW.to_string()),
        "FORE_BLUE" => Some(FORE_BLUE.to_string()),
        "FORE_MAGENTA" => Some(FORE_MAGENTA.to_string()),
        "FORE_CYAN" => Some(FORE_CYAN.to_string()),
        "FORE_WHITE" => Some(FORE_WHITE.to_string()),
        "FORE_LIGHTBLACK" => Some(FORE_LIGHTBLACK.to_string()),
        "FORE_LIGHTRED" => Some(FORE_LIGHTRED.to_string()),
        "FORE_LIGHTGREEN" => Some(FORE_LIGHTGREEN.to_string()),
        "FORE_LIGHTYELLOW" => Some(FORE_LIGHTYELLOW.to_string()),
        "FORE_LIGHTBLUE" => Some(FORE_LIGHTBLUE.to_string()),
        "FORE_LIGHTMAGENTA" => Some(FORE_LIGHTMAGENTA.to_string()),
        "FORE_LIGHTCYAN" => Some(FORE_LIGHTCYAN.to_string()),
        "FORE_LIGHTWHITE" => Some(FORE_LIGHTWHITE.to_string()),
        "FORE_RESET" => Some(FORE_RESET.to_string()),
        "BACK_BLACK" => Some(BACK_BLACK.to_string()),
        "BACK_RED" => Some(BACK_RED.to_string()),
        "BACK_GREEN" => Some(BACK_GREEN.to_string()),
        "BACK_YELLOW" => Some(BACK_YELLOW.to_string()),
        "BACK_BLUE" => Some(BACK_BLUE.to_string()),
        "BACK_MAGENTA" => Some(BACK_MAGENTA.to_string()),
        "BACK_CYAN" => Some(BACK_CYAN.to_string()),
        "BACK_WHITE" => Some(BACK_WHITE.to_string()),
        "BACK_LIGHTBLACK" => Some(BACK_LIGHTBLACK.to_string()),
        "BACK_LIGHTRED" => Some(BACK_LIGHTRED.to_string()),
        "BACK_LIGHTGREEN" => Some(BACK_LIGHTGREEN.to_string()),
        "BACK_LIGHTYELLOW" => Some(BACK_LIGHTYELLOW.to_string()),
        "BACK_LIGHTBLUE" => Some(BACK_LIGHTBLUE.to_string()),
        "BACK_LIGHTMAGENTA" => Some(BACK_LIGHTMAGENTA.to_string()),
        "BACK_LIGHTCYAN" => Some(BACK_LIGHTCYAN.to_string()),
        "BACK_LIGHTWHITE" => Some(BACK_LIGHTWHITE.to_string()),
        "BACK_RESET" => Some(BACK_RESET.to_string()),
        s => string_to_rgb(s),
    }
}
