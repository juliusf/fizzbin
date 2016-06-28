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
    let rom = read_binary(&rom_path);
    let mut chip8 = chip8::Chip8::new(rom);
    
    chip8.execute_rom();
    loop{

    }

}

fn read_binary<P: AsRef<Path>>(path: P) -> Box<[u8]>{
        let mut file = File::open(path).unwrap();
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).unwrap();
        file_buf.into_boxed_slice()
    }

fn init_graphics(){
}




