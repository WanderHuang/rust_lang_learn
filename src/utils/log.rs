use colored::*;

pub enum LogLevel {
    Info,
    Warn,
    Error,
}

impl LogLevel {
    pub fn note(self, args: Vec<&str>) {
        match self {
            LogLevel::Info => info(args),
            LogLevel::Warn => warn(args),
            LogLevel::Error  => error(args)
        }
    }
    
    pub fn note_def(self, args: Vec<&str>, label: &str) {
        match self {
            LogLevel::Info => info_def(args, label),
            LogLevel::Warn => warn_def(args, label),
            LogLevel::Error  => error_def(args, label)
        }
    }
}

pub fn info(args: Vec<&str>) {
    print_level(LogLevel::Info, args, "[ Info ]")
}

pub fn warn(args: Vec<&str>) {
    print_level(LogLevel::Warn, args, "[ Warn ]")
}

pub fn error(args: Vec<&str>) {
    print_level(LogLevel::Error, args, "[ Error ]")
}

/// 自定义info日志
pub fn info_def(args: Vec<&str>, label: &str) {
    print_level(LogLevel::Info, args, label)
}
/// 自定义warn日志
pub fn warn_def(args: Vec<&str>, label: &str) {
    print_level(LogLevel::Warn, args, label)
}
/// 自定义error日志
pub fn error_def(args: Vec<&str>, label: &str) {
    print_level(LogLevel::Error, args, label)
}
fn print_level(lv: LogLevel, args: Vec<&str>, tip: &str) {
    let label = match lv {
        LogLevel::Info => tip.blue(),
        LogLevel::Warn => tip.yellow(),
        LogLevel::Error => tip.red(),
    };
    print!("{} {}", label, ">".cyan());
    print_all(args);
}

fn print_all(args: Vec<&str>) {
    for arg in args {
        print!("{}", arg);
    }

    println!();
}
