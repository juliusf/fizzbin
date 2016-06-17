use std::env;
use std::io::Read;
use std::path::Path;
use std::fs::File;

mod cpu;
mod interconnect;

fn main() {
    let rom_path = env::args().nth(1).unwrap();
    let rom = read_binary(&rom_path);
    let mut interconnect = interconnect::Interconnect::new();
    interconnect.load_rom(rom);

    let mut cpu = cpu::Cpu::new(interconnect);
    cpu.run();

}

fn read_binary<P: AsRef<Path>>(path: P) -> Box<[u8]>{
    let mut file = File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf.into_boxed_slice()
}
