
const RAM_SIZE: usize = 4 * 1024;
const STACK_SIZE: usize = 16;

pub struct Interconnect{
    ram: Box<[u8]>,
    stack: Box<[u16]>
}

impl Interconnect{
    pub fn new() -> Interconnect{
        Interconnect{

        ram: vec![0; RAM_SIZE].into_boxed_slice(),
        stack: vec![0; STACK_SIZE].into_boxed_slice(),
        }
    }
    pub fn load_rom(&mut self, rom: Box<[u8]>){
         self.ram[0x200..(0x200 + rom.len())].clone_from_slice(&rom);
    }

    pub fn read_word(&mut self, addr: u16)->u16{
        if addr > 0x1ff && addr <= 0xfff {
            ((self.ram[addr as usize] as u16) << 8) |
            (self.ram[ (addr + 1) as usize] as u16)

        } else
        {
             panic!{"Invalid Memory access: {:#x}", addr}
        }

   }
}
