mod utils;

use utils::log::*;

fn main() {
    // println!("Hello, world!");
    info(["Hello World!"].to_vec());
    warn(["Hello World!"].to_vec());
    error(["Hello World!"].to_vec());
    info_def(["Hello World!"].to_vec(), "< CustomInfo");
    warn_def(["Hello World!"].to_vec(), "< CustomWarn");
    error_def(["Hello World!"].to_vec(), "< CustomError");

    LogLevel::Info.note(["You have a problem!"].to_vec());
    LogLevel::Warn.note(["You have a problem!"].to_vec());
    LogLevel::Error.note(["You have a problem!"].to_vec());

    LogLevel::Info.note_def(["You have a problem!"].to_vec(), "Fly");
    LogLevel::Warn.note_def(["You have a problem!"].to_vec(), "Run");
    LogLevel::Error.note_def(["You have a problem!"].to_vec(), "Dead");

}
