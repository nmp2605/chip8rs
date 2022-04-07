use crate::cpu::Cpu;
use crate::memory::Memory;

pub struct Motherboard {
    cpu: Cpu,
    memory: Memory,
}

impl Motherboard {
    pub fn initialize() -> Motherboard {
        Motherboard {
            cpu: Cpu::initialize(),
            memory: Memory::initialize(),
        }
    }

    pub fn emulate(&mut self, program: Vec<u8>) {
        self.memory.store_program(program);

        loop {
            self.cpu.fetch_and_decode(
                &self.memory,
            );
        }
    }
}