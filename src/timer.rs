use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

pub struct Timer{
	time: Arc<RwLock<u8>>
}

impl Timer{
	pub fn new() -> Timer{
		Timer{
			//time: RwLock::new(0),
			time: Arc::new(RwLock::new(15)),
		}
	}
	pub fn start_timer(&mut self){
		let arc = self.time.clone();
		thread::spawn( move || {
			let my_time = arc.clone();
            let sleep_time = Duration::from_millis(17);
            loop{
            	
            	let my_time_lock = my_time.read();
            	
                if *(my_time_lock.unwrap()) > 0{
                
                    let my_time_lock = my_time.write();
                /*    
                    match my_time_lock{
            			Err(e) =>{
            				panic!("ERR: {:?}", e);
            			},
            			_ =>{

            			}
            		}
				*/
                    let mut timer_val = my_time_lock.unwrap();
                    //println!("Updating lock to {}", *timer_val);
                    *timer_val = (*timer_val).wrapping_sub(1);

                }

                thread::sleep(sleep_time);
            }
        });
	}
	pub fn get(&mut self) -> u8{
		*self.time.read().unwrap()
	}

	pub fn set(&mut self, value: u8){
		let mut lock = self.time.write().unwrap();
		*lock = value;
	}
}