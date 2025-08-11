use atty::Stream;
use time::OffsetDateTime;
use crate::structs::colors;

fn timestamp_str() -> String {
    let now_utc = OffsetDateTime::now_utc();
    now_utc.unix_timestamp().to_string()
}

pub fn colorize(kind: &str, color_code: &str, use_color: bool) -> String {
    if !use_color {
        format!("[{}]", kind)
    } else {
        format!("{}[{}]{}", color_code, kind, colors::RESET)
    }
}

pub fn print_kind(kind: &str, color_code: &str, msg: String, is_err: bool) {
    let use_color = atty::is(if is_err { Stream::Stderr } else { Stream::Stdout });
    let k = colorize(kind, color_code, use_color);
    let ts = format!("[{}]", timestamp_str());
    if is_err {
        eprintln!("{ts} {k} {msg}");
    } else {
        println!("{ts} {k} {msg}");
    }
}

#[macro_export]
macro_rules! log {
    ($mode:literal, $($arg:tt)*) => {{
        let (label, color_code, is_err) = match $mode {
            "GW"   => ("GW", $crate::structs::colors::CYAN, false),
            "HB"   => ("HB", $crate::structs::colors::MAGENTA, false),
            "EVT"  => ("EVT", $crate::structs::colors::BLUE, false),
            "OK"   => ("OK", $crate::structs::colors::GREEN, false),
            "WARN" => ("WARN", $crate::structs::colors::YELLOW, true),
            "ERR"  => ("ERR", $crate::structs::colors::RED, true),
            "CLI"  => ("CLI", $crate::structs::colors::WHITE, false),
            _ => ("LOG", $crate::structs::colors::WHITE, false),
        };

        $crate::log::print_kind(label, color_code, format!($($arg)*), is_err)
    }};
}
