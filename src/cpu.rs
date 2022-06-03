use crate::instruction::Instruction;
use crate::memory::Memory;
use mockall_double::double;

#[double]
use crate::interface::Interface;

#[derive(Debug)]
pub struct Cpu {
    v_registers: [u8; 16],
    i_register: u16,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: usize,
    stack_empty: bool,
    stack_pointer: usize,
    stack: [u16; 0xF],
}

impl Cpu {
    pub fn initialize() -> Self {
        Cpu {
            v_registers: [0x0; 16],
            i_register: 0x0,
            delay_timer: 0x0,
            sound_timer: 0x0,
            program_counter: 0x200,
            stack_empty: true,
            stack_pointer: 0x0,
            stack: [0x0; 0xF],
        }
    }

    pub fn fetch_and_decode(&mut self, memory: &mut Memory, interface: &mut Interface) {
        let first_byte: u8 = memory.get(self.program_counter);

        self.increase_program_counter(0x1);

        let second_byte: u8 = memory.get(self.program_counter);

        self.increase_program_counter(0x1);

        println!("{:?}: {:x}{:x}", self.program_counter, first_byte, second_byte);

        Instruction::initialize(first_byte, second_byte)
            .interpret(self, memory, interface);
    }

    pub fn increase_program_counter(&mut self, amount: usize) {
        self.program_counter += amount;
    }

    pub fn decrease_program_counter(&mut self, amount: usize) {
        self.program_counter -= amount;
    }

    pub fn set_program_counter(&mut self, value: usize) {
        self.program_counter = value;
    }

    pub fn get_program_counter(&self) -> usize {
        self.program_counter
    }

    pub fn stack_push(&mut self, value: u16) {
        if self.stack_pointer == 0xF {
            panic!("The CPU has a stack overflow.");
        }

        if self.stack_empty {
            self.stack_empty = false;
        } else {
            self.stack_pointer += 1;
        }

        self.stack[self.stack_pointer] = value;
    }

    pub fn stack_pop(&mut self) -> u16 {
        if self.stack_empty == true {
            panic!("The CPU has a stack underflow.");
        }

        let stack_value: u16 = self.stack[self.stack_pointer];

        self.stack[self.stack_pointer] = 0x0;

        if self.stack_pointer == 0x0 {
            self.stack_empty = true;
        } else {
            self.stack_pointer -= 1;
        }

        stack_value
    }

    pub fn set_v_register(&mut self, register: usize, value: u8) {
        self.v_registers[register] = value;
    }

    pub fn get_v_register(&self, register: usize) -> u8 {
        self.v_registers[register]
    }

    pub fn set_i_register(&mut self, value: u16) {
        self.i_register = value;
    }

    pub fn get_i_register(&self) -> u16 {
        self.i_register
    }

    pub fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    pub fn get_delay_timer(&self) -> u8 {
        self.delay_timer
    }

    pub fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }

    pub fn get_sound_timer(&self) -> u8 {
        self.sound_timer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_initialize_the_cpu() {
        let cpu: Cpu = Cpu::initialize();

        assert_eq!([0x0; 16], cpu.v_registers);
        assert_eq!(0x0, cpu.i_register);
        assert_eq!(0x0, cpu.delay_timer);
        assert_eq!(0x0, cpu.sound_timer);
        assert_eq!(0x200, cpu.program_counter);
        assert_eq!(0x0, cpu.stack_pointer);
        assert_eq!([0x0; 0xF], cpu.stack);
    }

    #[test]
    fn it_should_increase_the_program_counter() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.increase_program_counter(0x4);

        assert_eq!(0x204, cpu.program_counter);
    }

    #[test]
    fn it_should_decrease_the_program_counter() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.program_counter = 0x204;

        cpu.decrease_program_counter(0x4);

        assert_eq!(0x200, cpu.program_counter);
    }

    #[test]
    fn it_should_set_a_value_to_the_program_counter() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_program_counter(0x5AA);

        assert_eq!(0x5AA, cpu.program_counter);
    }

    #[test]
    fn it_should_get_a_value_from_the_program_counter() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.program_counter = 0xAAA;

        assert_eq!(0xAAA, cpu.get_program_counter());
    }

    #[test]
    #[should_panic(expected = "The CPU has a stack overflow.")]
    fn it_should_fail_to_push_to_the_stack_if_it_is_overflowing() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.stack_pointer = 0xF;

        cpu.stack_push(0xFE);
    }

    #[test]
    fn it_should_push_to_an_empty_stack() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.stack_push(0xCA);
        
        assert_eq!(0xCA, cpu.stack[0x0]);
        assert_eq!(0x0, cpu.stack_pointer);
        assert_eq!(false, cpu.stack_empty);
    }

    #[test]
    fn it_should_push_to_an_initiated_stack() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.stack_push(0xCA);
        cpu.stack_push(0xFE);
        
        assert_eq!(0xCA, cpu.stack[0x0]);
        assert_eq!(0xFE, cpu.stack[0x1]);
        assert_eq!(0x1, cpu.stack_pointer);
        assert_eq!(false, cpu.stack_empty);
    }

    #[test]
    #[should_panic(expected = "The CPU has a stack underflow.")]
    fn it_should_fail_to_pop_from_an_empty_stack() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.stack_pop();
    }

    #[test]
    fn it_should_pop_from_the_last_position() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.stack_push(0xCA);

        let value: u16 = cpu.stack_pop();
        
        assert_eq!(0xCA, value);
        assert_eq!(0x0, cpu.stack_pointer);
        assert_eq!(true, cpu.stack_empty);
    }

    #[test]
    fn it_should_pop_from_another_position() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.stack_push(0xCA);
        cpu.stack_push(0xFE);

        let value: u16 = cpu.stack_pop();
        
        assert_eq!(0xFE, value);
        assert_eq!(0x0, cpu.stack_pointer);
        assert_eq!(false, cpu.stack_empty);
    }

    #[test]
    fn it_should_set_a_value_to_a_v_register() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_v_register(0x1, 0xCA);

        assert_eq!(0xCA, cpu.v_registers[0x1]);
    }

    #[test]
    fn it_should_read_a_value_from_a_v_register() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.v_registers[0xC] = 0xFE;

        assert_eq!(0xFE, cpu.get_v_register(0xC));
    }

    #[test]
    fn it_should_set_a_value_to_the_i_register() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_i_register(0xCAFE);

        assert_eq!(0xCAFE, cpu.i_register);
    }

    #[test]
    fn it_should_read_a_value_from_the_i_register() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.i_register = 0xBABE;

        assert_eq!(0xBABE, cpu.get_i_register());
    }

    #[test]
    fn it_should_set_a_value_to_the_delay_timer() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_delay_timer(0xCA);

        assert_eq!(0xCA, cpu.delay_timer);
    }

    #[test]
    fn it_should_read_a_value_from_the_delay_timer() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.delay_timer = 0xFE;

        assert_eq!(0xFE, cpu.get_delay_timer());
    }

    #[test]
    fn it_should_set_a_value_to_the_sound_timer() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_sound_timer(0xCA);

        assert_eq!(0xCA, cpu.sound_timer);
    }

    #[test]
    fn it_should_read_a_value_from_the_sound_timer() {
        let mut cpu: Cpu = Cpu::initialize();

        cpu.sound_timer = 0xFE;

        assert_eq!(0xFE, cpu.get_sound_timer());
    }
}