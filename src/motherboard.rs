use crate::cpu::Cpu;
use crate::memory::Memory;
use mockall_double::double;
use std::{thread, time};

#[double]
use crate::interface::Interface;

pub struct Motherboard {
    cpu: Cpu,
    interface: Interface,
    memory: Memory,
}

impl Motherboard {
    pub fn initialize() -> Self {
        Motherboard {
            cpu: Cpu::initialize(),
            interface: Interface::initialize(),
            memory: Memory::initialize(),
        }
    }

    pub fn emulate(&mut self, program: Vec<u8>) {
        self.memory.store_program(program);

        while self.interface.window_is_open() {
            self.cpu.fetch_and_decode(
                &mut self.memory, &mut self.interface
            );

            thread::sleep(time::Duration::from_millis(1500));
        }
    }
}