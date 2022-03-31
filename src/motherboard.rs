use crate::memory::Memory;

pub struct Motherboard {
    memory: Memory,
}

impl Motherboard {
    pub fn initialize() -> Motherboard {
        Motherboard {
            memory: Memory::initialize(),
        }
    }

    pub fn emulate() {
        
    }
}