use colored::*;
use std::env;
use std::path::Path;
use unicode_width::UnicodeWidthStr;
use whoami::{fallible, username};

// Old way
// fn get_shell() -> Option<String> {
//     if let Ok(shell) = env::var("SHELL") {
//         return Some(
//             Path::new(&shell)
//                 .file_name()?
//                 .to_string_lossy()
//                 .into_owned(),
//         );
//     }
//     None
// }

fn get_shell() -> Option<String> {
    env::var("SHELL").ok().and_then(|s| {
        Path::new(&s)
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
    })
}

fn get_host() -> Option<String> {
    fallible::hostname()
        .ok()
        .and_then(|s| if s.len() > 1 { Some(s) } else { None })
}

fn get_user() -> String {
    username()
}

fn get_desktop_env() -> Option<String> {
    let de_vars = ["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "GDMSESSION"];

    for de in de_vars {
        if let Ok(value) = env::var(de) {
            return Some(value);
        }
    }

    None
}

fn get_distro() -> String {
    let info = os_info::get();
    info.os_type().to_string()
}

fn center_text(text: &str, width: usize) -> String {
    let text_width = UnicodeWidthStr::width(text);
    if text_width >= width {
        text.to_string()
    } else {
        let padding = width - text_width;
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;
        format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
    }
}

fn main() {
    const COLUMN_WIDTH: usize = 17;

    let shell =
        center_text(&get_shell().unwrap_or_else(|| "".to_string()), COLUMN_WIDTH).bright_green();

    let host =
        center_text(&get_host().unwrap_or_else(|| "".to_string()), COLUMN_WIDTH).bright_yellow();

    let desktop_env = center_text(
        &get_desktop_env().unwrap_or_else(|| "Unknown DE".to_string()),
        COLUMN_WIDTH,
    )
    .bright_red();

    let user = center_text(&get_user(), COLUMN_WIDTH).bright_blue();
    let distro = center_text(&get_distro(), COLUMN_WIDTH).bright_magenta();

    let first = "   /\\   ".bright_cyan();
    let second = "  /  \\  ".bright_cyan();
    let third = " /____\\ ".bright_cyan();
    let fourth = " \\    / ".bright_cyan();
    let fifth = "  \\__/  ".bright_cyan();

    let separator_1 = "   ".yellow();
    let separator_2 = "   ".blue();
    let separator_3 = "   ".magenta();
    let separator_4 = "   ".red();
    let separator_5 = "   ".green();

    println!("{first}{separator_1}{host}{separator_1}{first}");
    println!("{second}{separator_2}{user}{separator_2}{second}");
    println!("{third}{separator_3}{distro}{separator_3}{third}");
    println!("{fourth}{separator_4}{desktop_env}{separator_4}{fourth}");
    println!("{fifth}{separator_5}{shell}{separator_5}{fifth}");
}
