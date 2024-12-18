use colored::*;
use os_info::get;
use std::fs::File;
use std::path::Path;
use std::{env, io::Read};
use unicode_width::UnicodeWidthStr;
use whoami::{fallible, username};
mod gems_art;

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

fn get_desktop_env() -> String {
    if env::var("DISPLAY").is_err() {
        return String::new();
    }

    for env_var in &[
        "XDG_SESSION_DESKTOP",
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION",
    ] {
        if let Ok(de) = env::var(env_var) {
            return de;
        }
    }

    let path = format!("{}/.xinitrc", env::var("HOME").unwrap_or_default());
    if let Ok(mut file) = File::open(&path) {
        let mut buf = String::new();
        if file.read_to_string(&mut buf).is_ok() {
            if let Some(last_line) = buf.lines().last() {
                let last_word = last_line.split(' ').last().unwrap_or("");
                return last_word.to_string();
            }
        }
    }

    String::from("N/A")
}

fn get_distro() -> String {
    let info = get();
    info.os_type().to_string()
}

fn center_text(text: &str, width: usize, total_width: usize) -> String {
    let text_width = UnicodeWidthStr::width(text);
    let padded_text = if text_width >= width {
        text.to_string()
    } else {
        let padding = width - text_width;
        let left_pad = padding / 2;
        let right_pad = padding - left_pad;
        format!("{}{}{}", " ".repeat(left_pad), text, " ".repeat(right_pad))
    };

    let total_padding = total_width - padded_text.len();
    let left_total_pad = total_padding / 2;
    let right_total_pad = total_padding - left_total_pad;

    format!(
        "{}{}{}",
        " ".repeat(left_total_pad),
        padded_text,
        " ".repeat(right_total_pad)
    )
}

fn main() {
    const COLUMN_WIDTH: usize = 17;
    const TOTAL_WIDTH: usize = 20;

    let args: Vec<String> = env::args().collect();
    let mut gem_style = "phos";

    // First check if the 1st argument is for help
    if args.len() > 1 && args[1] == "-h" {
        print_help();
        return;
    }

    if args.len() > 1 {
        gem_style = &args[1];
    }

    let shell = center_text(
        &get_shell().unwrap_or_else(|| "".to_string()),
        COLUMN_WIDTH,
        TOTAL_WIDTH,
    )
    .bright_green();

    let host = center_text(
        &get_host().unwrap_or_else(|| "".to_string()),
        COLUMN_WIDTH,
        TOTAL_WIDTH,
    )
    .bright_yellow();

    let desktop_env = center_text(&get_desktop_env(), COLUMN_WIDTH, TOTAL_WIDTH).bright_red();

    let user = center_text(&get_user(), COLUMN_WIDTH, TOTAL_WIDTH).bright_blue();

    let distro = center_text(&get_distro(), COLUMN_WIDTH, TOTAL_WIDTH).bright_magenta();

    let (first, second, third, fourth, fifth) = match gem_style {
        "phos" => (
            gems_art::phos::first(),
            gems_art::phos::second(),
            gems_art::phos::third(),
            gems_art::phos::fourth(),
            gems_art::phos::fifth(),
        ),
        "bort" => (
            gems_art::bort::first(),
            gems_art::bort::second(),
            gems_art::bort::third(),
            gems_art::bort::fourth(),
            gems_art::bort::fifth(),
        ),
        "cinn" => (
            gems_art::cinn::first(),
            gems_art::cinn::second(),
            gems_art::cinn::third(),
            gems_art::cinn::fourth(),
            gems_art::cinn::fifth(),
        ),
        "jade" => (
            gems_art::jade::first(),
            gems_art::jade::second(),
            gems_art::jade::third(),
            gems_art::jade::fourth(),
            gems_art::jade::fifth(),
        ),
        _ => (
            gems_art::phos::first(),
            gems_art::phos::second(),
            gems_art::phos::third(),
            gems_art::phos::fourth(),
            gems_art::phos::fifth(),
        ),
    };

    let separator_1 = "   ".yellow();
    let separator_2 = "   ".blue();
    let separator_3 = "   ".magenta();
    let separator_4 = "   ".red();
    let separator_5 = "   ".green();

    let terminal_height = 10;
    let content_height = 5;
    let top_padding = (terminal_height - content_height) / 2;
    let bottom_padding = terminal_height - content_height - top_padding;

    // Print top padding
    for _ in 0..top_padding {
        println!();
    }

    println!("{first}{separator_1}{host}{separator_1}{first}");
    println!("{second}{separator_2}{user}{separator_2}{second}");
    println!("{third}{separator_3}{distro}{separator_3}{third}");
    println!("{fourth}{separator_4}{desktop_env}{separator_4}{fourth}");
    println!("{fifth}{separator_5}{shell}{separator_5}{fifth}");

    for _ in 0..bottom_padding {
        println!();
    }
}

fn print_help() {
    println!("GemFetch - A program to display ASCII art based on gems :D.\n");
    println!("Usage:");
    println!("  GemFetch [options] [style]\n");
    println!("Options:");
    println!("  -h                Show this help message.");
    println!("\nAvailable styles:");
    println!("  phos            Classic Phosphophyllite gem.");
    println!("  bort            Bort hard diamond gem art.");
    println!("  cinn            Warm cinnabar gem art.");
    println!("  jade            Green gem jade art.");
    // Add more styles here if you define them.
    println!("\nExamples:");
    println!("  GemFetch -- -h");
    println!("  GemFetch classic");
    println!("  GemFetch modern");
}
