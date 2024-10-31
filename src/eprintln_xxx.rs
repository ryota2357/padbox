use colored::Colorize;

pub fn eprintln_error<S: AsRef<str>>(message: S) {
    eprintln!("{}: {}", "error".red(), message.as_ref());
}

pub fn eprintln_warning<S: AsRef<str>>(message: S) {
    eprintln!("{}: {}", "warn".yellow(), message.as_ref());
}

pub fn eprintln_info<S: AsRef<str>>(message: S) {
    eprintln!("{}: {}", "info".blue(), message.as_ref());
}

pub fn eprintln_tip<S: AsRef<str>>(message: S) {
    eprintln!("{}: {}", "tip".green(), message.as_ref());
}
