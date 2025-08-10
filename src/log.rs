use atty::Stream;
use owo_colors::OwoColorize;
use time::OffsetDateTime;
use time::macros::format_description; // <- importa el macro

#[derive(Clone, Copy)]
pub enum ColorKind {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
}

fn timestamp_str() -> String {
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    let fmt = format_description!("[month]-[day] [hour]:[minute]:[second]");
    now.format(fmt)
        .unwrap_or_else(|_| "?".into())
        .bright_black()
        .to_string()
}

fn colorize(kind: &str, color: ColorKind, use_color: bool) -> String {
    if !use_color {
        return format!("[{kind}]");
    }
    match color {
        ColorKind::Red => format!("[{}]", kind.red().bold()),
        ColorKind::Green => format!("[{}]", kind.green().bold()),
        ColorKind::Yellow => format!("[{}]", kind.yellow().bold()),
        ColorKind::Blue => format!("[{}]", kind.blue().bold()),
        ColorKind::Magenta => format!("[{}]", kind.magenta().bold()),
        ColorKind::Cyan => format!("[{}]", kind.cyan().bold()),
        ColorKind::White => format!("[{}]", kind.white().bold()),
        ColorKind::Gray => format!("[{}]", kind.bright_black().bold()),
    }
}

pub fn print_kind(kind: &str, color: ColorKind, msg: String, is_err: bool) {
    let use_color = atty::is(if is_err { Stream::Stderr } else { Stream::Stdout });
    let k = colorize(kind, color, use_color);
    let ts = format!("[{}]", timestamp_str());
    if is_err {
        eprintln!("{ts} {k} {msg}");
    } else {
        println!("{ts} {k} {msg}");
    }
}

#[macro_export]
macro_rules! log_gw {
    ($($arg:tt)*) => {
        $crate::log::print_kind("GW", $crate::log::ColorKind::Cyan, format!($($arg)*), false);
    }
}

#[macro_export]
macro_rules! log_hb {
    ($($arg:tt)*) => {
        $crate::log::print_kind("HB", $crate::log::ColorKind::Magenta, format!($($arg)*), false);
    }
}

#[macro_export]
macro_rules! log_evt {
    ($($arg:tt)*) => {
        $crate::log::print_kind("EVT", $crate::log::ColorKind::Blue, format!($($arg)*), false);
    }
}

#[macro_export]
macro_rules! log_ok {
    ($($arg:tt)*) => {
        $crate::log::print_kind("OK", $crate::log::ColorKind::Green, format!($($arg)*), false);
    }
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::log::print_kind("WARN", $crate::log::ColorKind::Yellow, format!($($arg)*), true);
    }
}

#[macro_export]
macro_rules! log_err {
    ($($arg:tt)*) => {
        $crate::log::print_kind("ERR", $crate::log::ColorKind::Red, format!($($arg)*), true);
    }
}

#[macro_export]
macro_rules! log_cli {
    ($($arg:tt)*) => {
        $crate::log::print_kind("CLI", $crate::log::ColorKind::White, format!($($arg)*), false);
    }
}
