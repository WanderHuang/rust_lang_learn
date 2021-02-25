extern crate test_lang;

#[macro_use]
extern crate log;

use test_lang::prelude::*;


fn main() {

    env_logger::init();

    info!("Logger is start!!");

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

    let mut man = Man {
        name: "wander",
        x: 0,
        y: 0,
        z: 0
    };

    man.step(1, 1);
    man.step(-1, 3);
    man.step(4, -4);

}
