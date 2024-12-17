use colored::*;
use std::env;
use std::path::Path;
use unicode_width::UnicodeWidthStr;

fn get_shell() -> Option<String> {
    if let Ok(shell) = env::var("SHELL") {
        return Some(
            Path::new(&shell)
                .file_name()?
                .to_string_lossy()
                .into_owned(),
        );
    }
    None
}

fn get_host() -> String {
    whoami::hostname()
}

fn get_user() -> String {
    whoami::username()
}

fn get_desktop_env() -> String {
    whoami::desktop_env().to_string()
}

fn get_distro() -> String {
    whoami::distro()
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
    let host = center_text(&get_host(), COLUMN_WIDTH).bright_yellow();
    let user = center_text(&get_user(), COLUMN_WIDTH).bright_blue();
    let desktop_env = center_text(&get_desktop_env(), COLUMN_WIDTH).bright_red();
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
