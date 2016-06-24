extern crate rand;

use super::{interconnect, timer};
use rand::{Rng, ThreadRng};


const NUM_GPR: usize = 16;

pub struct Cpu{
    reg_gpr: [u8; NUM_GPR],
    reg_I: u16,
    reg_pc: u16,
    reg_sp: u8,
    reg_DT: timer::Timer,
    reg_ST: timer::Timer,
    interconnect: interconnect::Interconnect,
    rng: ThreadRng,
}

impl Cpu{
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu{
        Cpu{
            reg_gpr: [0; NUM_GPR],
            reg_I: 0,
            reg_pc: 0x200, // TODO: is this correct?
            reg_sp: 0,
            reg_DT: timer::Timer::new(),
            reg_ST: timer::Timer::new(),
            interconnect: interconnect,
            rng: rand::thread_rng(),
        }
    }

    pub fn run(&mut self){
        self.reg_DT.start_timer();
        loop{
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self){
        let instruction = self.interconnect.read_word(self.reg_pc);
        let opcode = (instruction >> 12) & 0b1111;
        let VX = (instruction >> 8) & 0b00001111;
        let VY = (instruction >> 4) & 0x000F;

        match opcode{
            0x1 =>{
                //JMP addr
                let addr =  (instruction) & 0x0FFF;
                self.reg_pc = addr;
                return
            },
            0x2 => {
                // CALL Addr
                let addr = (instruction) & 0b0000111111111111;
                self.interconnect.push_stack(self.reg_pc + 2);
                self.reg_pc = addr;
                return
            },
            0x3 => {
                // SE VX == byte
                let value = (instruction) & 0x00FF;
                if value as u8 == self.reg_gpr[ VX as usize]{
                     self.reg_pc +=2;
                }
            },
            0x4 => {
                //SE VX != byte
                let value = (instruction) & 0x00FF;
                if value as u8 != self.reg_gpr[ VX as usize]{
                     self.reg_pc +=2;
                }
            },
            0x6 => {
                 //Set VX = Byte
                let value = (instruction ) & 0x00FF;
                self.write_reg_gpr(VX as usize, value as u8)
            },
            0x7 => {
                // add vr byte
                let value = (instruction) & 0x00FF;
                let new_value = self.reg_gpr[VX as usize].wrapping_add(value as u8);
                self.write_reg_gpr(VX as usize, new_value)
            },
            0x8 => {
                match instruction & 0xF00F{
                    0x8000 =>{
                        //LD Vx, vy
                        let value_y = self.reg_gpr[VY as usize];
                        self.write_reg_gpr(VX as usize, value_y as u8);
                    },
                    _ => {
                        self.halt_invalid_instruction(opcode, instruction);
                    }
                }
            },
            0x9 => {
                // SNE Vx, Vy
                if VX != VY{
                    self.reg_pc += 2;
                }
            },
            0xa => {
                //LD I,Addr
                let addr = (instruction) & 0b0000111111111111;
                self.reg_I = addr;
            },
            0xc =>{
                 // RND VX, Byte
                let rand = self.rng.gen::<u8>();
                let value = (instruction) & 0x00FF;
                self.write_reg_gpr(VX as usize, rand & value as u8);
            },
            0xd => {
                //DRW Vx, Vy, nibble
                let x_reg = (instruction >> 8) &  0b00001111;
                let y_reg = (instruction >> 4) &  0x000F;
                let x_coord = self.reg_gpr[x_reg as usize];
                let y_coord = self.reg_gpr[y_reg as usize];
                let nibble  = (instruction) & 0x000F;
                let collision = self.interconnect.draw_on_screen(x_coord as usize, y_coord  as usize, self.reg_I, nibble as usize);
                if collision == true {
                     self.reg_gpr[15] = 1;
                }else{
                     self.reg_gpr[15] = 0;
                }
                println!("x coord: {:#x}, y coord: {:#x}", x_coord, y_coord);

            },
            0x0 => {
                match instruction {
                    0x00EE =>{
                        // return from subroutine call
                        self.reg_pc = self.interconnect.pop_stack();
                        return
                    },
                    _ => {
                        self.halt_invalid_instruction(opcode, instruction);
                    }

               }
            },
            0xe => {
                match instruction & 0xF0FF{
                    0xE0A1 => {
                        //SKNP Vx
                        let key_pressed = self.interconnect.get_key_state(VX as usize);
                        if ! key_pressed{
                            self.reg_pc += 2;
                        }
                    },
                    0xE09E => {
                        // SKP Vx
                        let key_pressed = self.interconnect.get_key_state(VX as usize);
                        if key_pressed{
                            self.reg_pc += 2;
                        }

                    },
                    _ =>{
                        self.halt_invalid_instruction(opcode, instruction);
                    }
                }
            },
            0xf =>{
                match instruction & 0xF0FF{
                    0xF01E => {
                        //ADD I, VX
                        let mut tmp = self.reg_I;
                        tmp += self.reg_gpr[VX as usize] as u16;
                        self.reg_I = tmp;
                    },
                    0xF015 => {
                        // LD DT, VX
                        self.reg_DT.set(VX as u8);
                    },
                    0xF007 =>{
                        //LD Vx, DT
                        let dt = self.reg_DT.get();
                        self.write_reg_gpr(VX as usize, dt as u8);
                    },

                    0xF055 => {
                        //LD [I], Vx
                        let offset = self.reg_I;
                        for i in 0 .. VX{
                            let register_val = self.reg_gpr[i as usize];
                            self.interconnect.write_byte_to_ram(offset + i, register_val);
                        }
                    },
                    _=> {
                        self.halt_invalid_instruction(opcode, instruction);
                    }

                }
            },

            _ =>{
                self.halt_invalid_instruction(opcode, instruction);
            }
        }

        self.reg_pc += 2;
    }

    fn halt_invalid_instruction(&mut self, opcode: u16, instruction: u16){
        panic!("Invalid Instruction! opcode: {:#x} instruction: {:#x}", opcode, instruction);
    }

    fn write_reg_gpr(&mut self, index: usize, value:u8 )
    {
        if index != 0xF && index <= NUM_GPR
        {
        self.reg_gpr[index] = value
        } else
        {
             panic!("Invalid Register access!");
        }

    }
}
