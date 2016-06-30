use super::gfx;
use super::inputhandler;
use std::sync::{Arc,Mutex};

const RAM_SIZE: usize = 4 * 1024;
const STACK_SIZE: usize = 16;

pub struct Interconnect{
    ram: Box<[u8]>,
    stack: Box<[u16]>,
    stack_ptr: usize,
    gfx: gfx::Gfx,
    input_handler: inputhandler::InputHandler, 
}

impl Interconnect{
    pub fn new() -> Interconnect{
        Interconnect{

        ram: vec![0; RAM_SIZE].into_boxed_slice(),
        stack: vec![0; STACK_SIZE].into_boxed_slice(),
        stack_ptr:  0,
        gfx: gfx::Gfx::new(),
        input_handler: inputhandler::InputHandler::new(),
        }
    }
    pub fn load_rom(&mut self, rom: &Box<[u8]>){
         self.ram[0x200..(0x200 + rom.len())].clone_from_slice(&rom);
    }

    pub fn read_word(&mut self, addr: u16)->u16{
        if addr > 0x1ff && addr <= 0xfff {
            ((self.ram[addr as usize] as u16) << 8) |
            (self.ram[ (addr + 1) as usize] as u16)

        } else
        {
             panic!("Invalid Memory access: {:#x}", addr)
        }
    }

    pub fn push_stack(&mut self, addr: u16){
        if self.stack_ptr + 1 == STACK_SIZE{
             panic!("Stack Overflow!");
        }
        self.stack_ptr +=1;
        self.stack[self.stack_ptr] = addr;

    }

    pub fn draw_on_screen(&mut self, screen_loc_x: usize, screen_loc_y: usize, memory_loc: u16, length: usize) -> bool{

        let sprite = &self.ram[memory_loc as usize .. (memory_loc + length as u16) as usize];
        let result = self.gfx.draw_sprite(screen_loc_x, screen_loc_y, &sprite);
        //print!("{:?}", self.gfx);
        result
    }

    pub fn get_key_state(&mut self, key_index: usize) -> bool{
        self.input_handler.get_key_state(key_index)
    }

    pub fn write_byte_to_ram(&mut self, addr: u16, val: u8){
        self.ram[addr as usize] = val;
    }

    pub fn read_byte_from_ram(&mut self, addr:u16) -> u8{
        self.ram[addr as usize]
    }

    pub fn get_screen_contents(&mut self) -> Arc<Mutex<[[bool; gfx::RES_X]; gfx::RES_Y]>>{
         self.gfx.get_frame()
    }

    pub fn pop_stack(&mut self) -> u16
    {
        if self.stack_ptr >= 0{
            self.stack_ptr -=1;
            self.stack[self.stack_ptr +1]
       }else
       {
            panic!("Called pop on empty stack!");
       }
    }
}
