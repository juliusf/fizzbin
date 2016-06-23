use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

const NUM_KEYS: usize = 16;

pub struct InputHandler{
	io_state: Arc<RwLock<[bool; NUM_KEYS]>>,
}

impl InputHandler {
	pub fn new() -> InputHandler{
		InputHandler{
			io_state: Arc::new(RwLock::new([false; NUM_KEYS]))
		}
	}

	pub fn start_input_handler(&mut self){
		// TODO
	}

	pub fn get_key_state(&mut self, key_index: usize) -> bool{
		let arr = *self.io_state.read().unwrap();
		arr[key_index]
	}
}