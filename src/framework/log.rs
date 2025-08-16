use crate::structs::colors;
use std::io::IsTerminal;
use time::{OffsetDateTime, UtcOffset, macros::format_description};

#[derive(Debug, Clone, Copy)]
pub enum LogKind {
    GW,
    HB,
    EVT,
    OK,
    WARN,
    ERR,
    CLI,
    LOG,
}

impl std::fmt::Display for LogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            LogKind::GW => "GW",
            LogKind::HB => "HB",
            LogKind::EVT => "EVT",
            LogKind::OK => "OK",
            LogKind::WARN => "WARN",
            LogKind::ERR => "ERR",
            LogKind::CLI => "CLI",
            LogKind::LOG => "LOG",
        })
    }
}

impl From<&str> for LogKind {
    fn from(s: &str) -> Self {
        match s {
            "GW" => Self::GW,
            "HB" => Self::HB,
            "EVT" => Self::EVT,
            "OK" => Self::OK,
            "WARN" => Self::WARN,
            "ERR" => Self::ERR,
            "CLI" => Self::CLI,
            _ => Self::LOG,
        }
    }
}

fn timestamp_str() -> String {
    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let now_local = OffsetDateTime::now_utc().to_offset(offset);
    let format = format_description!("[month]-[day]-[year] [hour]:[minute]");
    now_local.format(&format).unwrap()
}

pub fn colorize(kind: LogKind, color_code: &str, use_color: bool) -> String {
    if !use_color {
        format!("[{}]", kind)
    } else {
        format!("{}[{}]{}", color_code, kind, colors::RESET)
    }
}

pub fn print_kind(kind: LogKind, color_code: &str, msg: String, is_err: bool) {
    let use_color = if is_err {
        std::io::stderr().is_terminal()
    } else {
        std::io::stdout().is_terminal()
    };
    let k = colorize(kind, color_code, use_color);
    let ts = format!(
        "{}[{}]{}",
        colors::DARK_GRAY,
        timestamp_str(),
        colors::RESET
    );
    if is_err {
        eprintln!("{ts} {k} {msg}");
    } else {
        println!("{ts} {k} {msg}");
    }
}

#[macro_export]
macro_rules! log {
    ($mode:expr, $($arg:tt)*) => {{
        let mode: $crate::framework::log::LogKind = ($mode).into();
        let (color_code, is_err) = match mode {
            $crate::framework::log::LogKind::GW   => ($crate::structs::colors::CYAN,    false),
            $crate::framework::log::LogKind::HB   => ($crate::structs::colors::MAGENTA, false),
            $crate::framework::log::LogKind::EVT  => ($crate::structs::colors::BLUE,    false),
            $crate::framework::log::LogKind::OK   => ($crate::structs::colors::GREEN,   false),
            $crate::framework::log::LogKind::WARN => ($crate::structs::colors::YELLOW,  true),
            $crate::framework::log::LogKind::ERR  => ($crate::structs::colors::RED,     true),
            $crate::framework::log::LogKind::CLI  => ($crate::structs::colors::WHITE,   false),
            $crate::framework::log::LogKind::LOG  => ($crate::structs::colors::WHITE,   false),
        };
        $crate::framework::log::print_kind(mode, color_code, format!($($arg)*), is_err);
    }};
}

pub struct Log;
impl Log {
    pub fn ok(msg: impl AsRef<str>) {
        log!(LogKind::OK, "{}", msg.as_ref());
    }
    pub fn warn(msg: impl AsRef<str>) {
        log!(LogKind::WARN, "{}", msg.as_ref());
    }
    pub fn err(msg: impl AsRef<str>) {
        log!(LogKind::ERR, "{}", msg.as_ref());
    }
}
