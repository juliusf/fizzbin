extern crate piston_window;
extern crate rand;

use std::env;
use std::io::Read;
use std::path::Path;
use std::fs::File;
use std::thread;
use piston_window::*;

use std::sync::{Arc,RwLock,Mutex};
use std::time::Duration;

mod gfx;
mod cpu;
mod interconnect;
mod timer;
mod inputhandler;
mod chip8;

fn main() {
    println!("init");
    init_graphics();
    let rom_path = env::args().nth(1).unwrap();
    let rom = read_binary(&rom_path);
    let mut chip8 = chip8::Chip8::new(rom);
    
    let sleep_time = Duration::from_millis(500);
    chip8.execute_rom();
    
    loop{
        {
        println!("trying to get contents");
        let foo = chip8.get_screen_contents();
        println!("got contents");
        //panic!("lock1 released");
        draw_screen(foo);
        thread::sleep(sleep_time);
        }
    }

}

fn draw_screen( gfx: Arc<std::sync::Mutex<[[bool; 64]; 32]>>)
{
    let data = gfx.lock().unwrap();
    println!("got data!");
    println!("ping");
}

fn read_binary<P: AsRef<Path>>(path: P) -> Box<[u8]>{
        let mut file = File::open(path).unwrap();
        let mut file_buf = Vec::new();
        file.read_to_end(&mut file_buf).unwrap();
        file_buf.into_boxed_slice()
    }

fn init_graphics(){

}




