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
}
