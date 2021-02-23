extern crate test_lang;

use test_lang::prelude::*;


fn main() {
    let mut man = Man {
        name: "wander",
        x: 0,
        y: 0,
        z: 0
    };

    man.step(1, 1);
    man.step(-1, 3);
    man.step(4, -4);


    man.cry("me");

}
