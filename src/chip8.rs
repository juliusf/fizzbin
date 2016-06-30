
use super::{cpu, interconnect};
use std::sync::{Arc,RwLock,Mutex};
use std::thread;

use std::io::Read;
use std::path::Path;
use std::fs::File;
use super::gfx;

pub struct Chip8{
	interconnect: Arc<Mutex<interconnect::Interconnect>>,
	rom: Arc<Box<[u8]>>
}
impl Chip8{
	pub fn new(rom: Box<[u8]>) -> Chip8{
		Chip8{
			interconnect: Arc::new(Mutex::new(interconnect::Interconnect::new())),
			rom: Arc::new(rom)		
		}
	}

	pub fn execute_rom(&mut self){
		let  my_interconnect_mutex = self.interconnect.clone();
		let my_rom = self.rom.clone();
		thread::spawn(move || { 
			let mut my_interconnect = my_interconnect_mutex.lock().unwrap();
			my_interconnect.load_rom(& my_rom);
			let mut cpu = cpu::Cpu::new(&mut my_interconnect); 
			cpu.run();
		});
	}

	pub fn get_screen_contents(&mut self) -> Arc<Mutex<[[bool; gfx::RES_X]; gfx::RES_Y]>>{
		self.interconnect.lock().unwrap().get_screen_contents()
	} 
}