
use super::{cpu, interconnect};
use std::sync::{Arc,RwLock};
use std::thread;

use std::io::Read;
use std::path::Path;
use std::fs::File;

pub struct Chip8{
	interconnect: Arc<RwLock<interconnect::Interconnect>>,
}
impl Chip8{
	pub fn new() -> Chip8{
		Chip8{
			interconnect: Arc::new(RwLock::new(interconnect::Interconnect::new())),
		}
	}

	pub fn execute_rom(&mut self){
		let mut my_interconnect_lock = self.interconnect.clone();
		thread::spawn(move || {

			let mut rom = Chip8::read_binary("roms/TETRIS");
			let mut my_interconnect = my_interconnect_lock.write().unwrap();
			let mut cpu = cpu::Cpu::new(&mut *my_interconnect); 
			cpu.run();
		}); 
	}

	fn read_binary<P: AsRef<Path>>(path: P) -> Box<[u8]>{
    	let mut file = File::open(path).unwrap();
    	let mut file_buf = Vec::new();
    	file.read_to_end(&mut file_buf).unwrap();
    	file_buf.into_boxed_slice()
	}
}