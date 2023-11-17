enum Ansi {
    Red,
    Green,
    Yellow,
    Magenta,
}

impl Ansi {
    fn apply(self: Ansi, s: &str) -> String {
        let colour = match self {
            Ansi::Red => 91,
            Ansi::Green => 92,
            Ansi::Yellow => 93,
            Ansi::Magenta => 95,
        };

        const RESET: &str = "\x1b[0m";
        format!("\x1b[{colour}m{s}{RESET}")
    }
}

pub(crate) fn info(message: &str) {
    log(Ansi::Green.apply("INFO   "), message);
}

pub(crate) fn warn(message: &str) {
    log(Ansi::Yellow.apply("WARN   "), message);
}

pub(crate) fn error(message: &str) {
    log(Ansi::Red.apply("ERROR  "), message);
}

pub(crate) fn debug(item: &impl std::fmt::Debug) {
    let level = Ansi::Magenta.apply("DEBUG  ");
    println!("[{level}] {item:?}")
}

fn log(level: String, message: &str) {
    println!("[{level}] {message}")
}
