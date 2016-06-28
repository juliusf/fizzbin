extern crate piston_window;
extern crate rand;

use std::env;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::thread;
use piston_window::*;

mod gfx;
mod cpu;
mod interconnect;
mod timer;
mod inputhandler;
mod chip8;

fn main() {

    init_graphics();
    let rom_path = env::args().nth(1).unwrap();
    //let mut rom = read_binary(&rom_path);
    /*
    let mut interconnect = interconnect::Interconnect::new();
    interconnect.load_rom(rom);

    let mut cpu = cpu::Cpu::new(interconnect);
    cpu.run();
    */
    let mut chip8 = chip8::Chip8::new();
    chip8.execute_rom();
    loop{

    }

}

fn init_graphics(){
}




