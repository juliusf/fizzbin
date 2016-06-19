use super::interconnect;
const NUM_GPR: usize = 16;

pub struct Cpu{
    reg_gpr: [u8; NUM_GPR],
    reg_I: u16,
    reg_pc: u16,
    reg_sp: u8,
    reg_DT: u16, // TODO: other data type
    reg_ST: u16,  // TODO: other data type

    interconnect: interconnect::Interconnect
}

impl Cpu{
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu{
        Cpu{
            reg_gpr: [0; NUM_GPR],
            reg_I: 0,
            reg_pc: 0x200, // TODO: is this correct?
            reg_sp: 0,
            reg_DT: 0,
            reg_ST: 0,
            interconnect: interconnect
        }
    }

    pub fn run(&mut self){
        loop{
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self){
        let instruction = self.interconnect.read_word(self.reg_pc);
        println!("instruction: {:#x}", instruction);
        let opcode = (instruction >> 12) & 0b1111;
        println!("opcode: {:#x}", opcode);
        let VX = (instruction >> 8) & 0b00001111;

        match opcode{
            0x2 => {
                // CALL Addr
                let addr = (instruction) & 0b0000111111111111;
                self.interconnect.push_stack(self.reg_pc + 2);
                self.reg_pc = addr;
                return
            },
            0x6 => {
                 //Set VX = Byte
                let value = (instruction ) & 0x00FF;
                self.write_reg_gpr(VX as usize, value as u8)
            },
            0x7 => {
                // add vr,vx
                let value = (instruction) & 0x00FF;
                let new_value = self.reg_gpr[VX as usize] + value as u8;
                self.write_reg_gpr(VX as usize, new_value)
            },
            0xa => {
                //LD I,Addr
                let addr = (instruction) & 0b0000111111111111;
                self.reg_I = addr;
                println!("addr: {:#x}", addr);
            },
            0x0 => {
                match instruction {
                    0x00EE =>{
                        // return from subroutine call
                        self.reg_pc = self.interconnect.pop_stack();
                        return
                    },
                    _ => {
                        panic!("Invalid Instrucition!");
                    }

               }
            },

            _ =>{
                panic!("Unrecognized instruction: {:#x}", instruction);
            }
        }

        self.reg_pc += 2;
    }

    fn write_reg_gpr(&mut self, index: usize, value:u8 )
    {
        if (index != 0xF && index <= NUM_GPR)
        {
        self.reg_gpr[index] = value
        } else
        {
             panic!("Invalid Register access!");
        }

    }
}
