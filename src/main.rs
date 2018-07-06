#![feature(exclusive_range_pattern)]

#[macro_use] extern crate arrayref;
#[macro_use] extern crate lazy_static;
extern crate sdl2;

mod nes;
use nes::Nes;

fn main() {
    let mut nes = Nes::new("rom/hello_world.nes");
    nes.run();
}
