pub mod phos {
    use colored::*;
    pub fn first() -> String {
        "   /\\   ".bright_cyan().to_string()
    }

    pub fn second() -> String {
        "  /  \\  ".bright_cyan().to_string()
    }

    pub fn third() -> String {
        " /____\\ ".bright_cyan().to_string()
    }

    pub fn fourth() -> String {
        " \\    / ".bright_cyan().to_string()
    }

    pub fn fifth() -> String {
        "  \\__/  ".bright_cyan().to_string()
    }
}

pub mod bort {
    use colored::*;

    pub fn first() -> String {
        "   /\\   ".bright_black().to_string()
    }

    pub fn second() -> String {
        "  /==\\  ".bright_black().to_string()
    }

    pub fn third() -> String {
        " /====\\ ".bright_black().to_string()
    }

    pub fn fourth() -> String {
        " \\====/ ".bright_black().to_string()
    }

    pub fn fifth() -> String {
        "  \\==/  ".bright_black().to_string()
    }
}

pub mod cinn {
    use colored::*;

    pub fn first() -> String {
        "   ()   ".bright_red().to_string()
    }

    pub fn second() -> String {
        "  (__)  ".bright_red().to_string()
    }

    pub fn third() -> String {
        " (____) ".bright_red().to_string()
    }

    pub fn fourth() -> String {
        " \\____/ ".bright_red().to_string()
    }

    pub fn fifth() -> String {
        "  \\__/  ".bright_red().to_string()
    }
}

pub mod jade {
    use colored::*;

    pub fn first() -> String {
        "   /\\   ".bright_green().to_string()
    }

    pub fn second() -> String {
        "  /\\/\\  ".bright_green().to_string()
    }

    pub fn third() -> String {
        " /\\/\\/\\ ".bright_green().to_string()
    }

    pub fn fourth() -> String {
        " \\/\\/\\/ ".bright_green().to_string()
    }

    pub fn fifth() -> String {
        "  \\__/  ".bright_green().to_string()
    }
}
