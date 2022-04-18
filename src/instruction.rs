use std::ops::{Div, Sub};

use crate::cpu::Cpu;
use crate::memory::Memory;
use mockall_double::double;
use rand::Rng;

#[double]
use crate::interface::Interface;

pub struct Instruction {
    opcode: u16,
}

impl Instruction {
    pub fn initialize(first_byte: u8, second_byte: u8) -> Self {
        Instruction {
            opcode: u16::from(first_byte) << 8 | u16::from(second_byte),
        }
    }

    pub fn interpret(&mut self, cpu: &mut Cpu, memory: &mut Memory, interface: &mut Interface) {
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode {
                0x00E0 => self.clear_display(interface),
                0x00EE => self.return_from_subroutine(cpu),
                _ => (),
            },
            0x1000 => self.jump_to_address(cpu),
            0x2000 => self.call_subroutine_at_address(cpu),
            0x3000 => self.skip_next_instruction_if_argument_equals_v_register_value(cpu),
            0x4000 => self.skip_next_instruction_if_argument_differs_v_register_value(cpu),
            0x5000 => self.skip_next_instruction_if_v_registers_values_are_the_same(cpu),
            0x6000 => self.put_argument_value_on_v_register(cpu),
            0x7000 => self.add_argument_value_to_v_register(cpu),
            0x8000 => match self.opcode & 0xF00F {
                0x8000 => self.put_v_register_value_on_other_v_register(cpu),
                0x8001 => self.put_value_of_bitwise_or_operation_between_v_registers_on_first_passed_register(cpu),
                0x8002 => self.put_value_of_bitwise_and_operation_between_v_registers_on_first_passed_register(cpu),
                0x8003 => self.put_value_of_bitwise_xor_operation_between_v_registers_on_first_passed_register(cpu),
                0x8004 => self.put_value_of_sum_operation_between_v_registers_on_first_passed_register(cpu),
                0x8005 => self.put_value_of_subtraction_operation_between_v_registers_on_first_passed_register(cpu),
                0x8006 => self.put_value_of_bitwise_shift_right_operation_between_v_registers_on_first_passed_register(cpu),
                0x8007 => self.put_value_of_inverted_subtraction_operation_between_v_registers_on_first_passed_register(cpu),
                0x800E => self.put_value_of_bitwise_shift_left_operation_between_v_registers_on_first_passed_register(cpu),
                _ => (),
            },
            0x9000 => self.skip_next_instruction_if_v_registers_values_are_different(cpu),
            0xA000 => self.put_argument_value_on_i_register(cpu),
            0xB000 => self.jump_to_argument_value_plus_v0(cpu),
            0xC000 => self.put_value_of_bitwise_or_operation_between_argument_and_random_byte_on_passed_v_register(cpu),
            0xD000 => self.draw_byte_sprite_starting_at_location_i_on_register_stored_location(cpu),
            0xE000 => match self.opcode & 0xF0FF {
                0xE09E => self.skip_next_instruction_if_key_with_v_register_value_is_pressed(cpu),
                0xE0A1 => self.skip_next_instruction_if_key_with_v_register_value_is_not_pressed(cpu),
                _ => (),
            }
            0xF000 => match self.opcode & 0xF0FF {
                0xF007 => self.put_delay_timer_value_on_v_register(cpu),
                0xF00A => self.wait_for_key_press_and_store_value_on_v_register(cpu),
                0xF015 => self.put_v_register_value_on_delay_timer(cpu),
                0xF018 => self.put_v_register_value_on_sound_timer(cpu),
                0xF01E => self.add_v_register_value_to_i_register_value(cpu),
                0xF029 => self.put_location_of_sprite_for_v_register_digit_on_i_register(cpu),
                0xF033 => self.put_bcd_representation_of_v_register_in_memory_locations_starting_on_i_register_location(cpu, memory),
                0xF055 => self.put_values_of_v_registers_from_v0_to_passed_v_register_in_memory_starting_on_i_register_location(cpu, memory),
                0xF065 => self.put_values_on_v_registers_from_v0_to_passed_v_register_from_memory_starting_on_i_register_location(cpu, memory),
                _ => (),
            },
            _ => (),
        }
    }

    fn clear_display(&self, interface: &mut Interface) {
        println!("clear_display");

        interface.set_buffer(
            vec!(Interface::BLACK; Interface::WIDTH * Interface::HEIGHT)
        );
    }

    fn return_from_subroutine(&self, cpu: &mut Cpu) {
        println!("return_from_subroutine");

        let top_value_from_stack: usize = cpu.stack_pop() as usize;

        cpu.set_program_counter(top_value_from_stack);
    }

    fn jump_to_address(&self, cpu: &mut Cpu) {
        println!("jump_to_address");

        let address = usize::from(self.opcode & 0x0FFF);

        cpu.set_program_counter(address);
    }

    fn call_subroutine_at_address(&self, cpu: &mut Cpu) {
        println!("call_subroutine_at_address");

        cpu.stack_push(
            cpu.get_program_counter() as u16,
        );

        let address = usize::from(self.opcode & 0x0FFF);

        cpu.set_program_counter(address);
    }

    fn skip_next_instruction_if_argument_equals_v_register_value(&self, cpu: &mut Cpu) {
        println!("skip_next_instruction_if_argument_equals_v_register_value");

        let argument_value: u8 = (self.opcode as u8) & 0x00FF;
        let register_number: usize = (self.opcode as usize) >> 8 & 0x000F;

        let register_value: u8 = cpu.get_v_register(register_number);

        if argument_value == register_value {
            cpu.increase_program_counter(2);
        }
    }

    fn skip_next_instruction_if_argument_differs_v_register_value(&self, cpu: &mut Cpu) {
        println!("skip_next_instruction_if_argument_differs_v_register_value");

        let argument_value: u8 = (self.opcode as u8) & 0x00FF;
        let register_number: usize = (self.opcode as usize) >> 8 & 0x000F;

        let register_value: u8 = cpu.get_v_register(register_number);

        if argument_value != register_value {
            cpu.increase_program_counter(2);
        }
    }

    fn skip_next_instruction_if_v_registers_values_are_the_same(&self, cpu: &mut Cpu) {
        println!("skip_next_instruction_if_v_registers_values_are_the_same");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);
        let second_register_value: u8 = cpu.get_v_register(second_register_number);

        if first_register_value == second_register_value {
            cpu.increase_program_counter(2);
        }
    }

    fn put_argument_value_on_v_register(&self, cpu: &mut Cpu) {
        println!("put_argument_value_on_v_register");

        let argument_value: u8 = (self.opcode as u8) & 0x00FF;
        let register_number: usize = (self.opcode as usize) >> 8 & 0x000F;

        cpu.set_v_register(register_number, argument_value);
    }

    fn add_argument_value_to_v_register(&self, cpu: &mut Cpu) {
        println!("add_argument_value_to_v_register");

        let argument_value: u8 = (self.opcode as u8) & 0x00FF;
        let register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let register_value: u8 = cpu.get_v_register(register_number);

        cpu.set_v_register(
            register_number, 
            register_value.wrapping_add(argument_value)
        );
    }

    fn put_v_register_value_on_other_v_register(&self, cpu: &mut Cpu) {
        println!("put_v_register_value_on_other_v_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        cpu.set_v_register(
            first_register_number, 
            cpu.get_v_register(second_register_number)
        );
    }

    fn put_value_of_bitwise_or_operation_between_v_registers_on_first_passed_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_bitwise_or_operation_between_v_registers_on_first_passed_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);
        let second_register_value: u8 = cpu.get_v_register(second_register_number);

        cpu.set_v_register(
            first_register_number, 
            first_register_value | second_register_value
        );
    }

    fn put_value_of_bitwise_and_operation_between_v_registers_on_first_passed_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_bitwise_and_operation_between_v_registers_on_first_passed_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);
        let second_register_value: u8 = cpu.get_v_register(second_register_number);

        cpu.set_v_register(
            first_register_number, 
            first_register_value & second_register_value
        );
    }

    fn put_value_of_bitwise_xor_operation_between_v_registers_on_first_passed_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_bitwise_xor_operation_between_v_registers_on_first_passed_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);
        let second_register_value: u8 = cpu.get_v_register(second_register_number);

        cpu.set_v_register(
            first_register_number, 
            first_register_value ^ second_register_value
        );
    }

    fn put_value_of_sum_operation_between_v_registers_on_first_passed_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_sum_operation_between_v_registers_on_first_passed_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);
        let second_register_value: u8 = cpu.get_v_register(second_register_number);

        let (value, flag): (u8, bool) = first_register_value.overflowing_add(second_register_value);

        cpu.set_v_register(0xF, flag as u8);
        cpu.set_v_register(first_register_number, value);
    }

    fn put_value_of_subtraction_operation_between_v_registers_on_first_passed_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_subtraction_operation_between_v_registers_on_first_passed_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);
        let second_register_value: u8 = cpu.get_v_register(second_register_number);

        let (value, flag): (u8, bool) = first_register_value.overflowing_sub(second_register_value);

        cpu.set_v_register(0xF, !flag as u8);
        cpu.set_v_register(first_register_number, value);
    }

    fn put_value_of_bitwise_shift_right_operation_between_v_registers_on_first_passed_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_bitwise_shift_right_operation_between_v_registers_on_first_passed_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);

        let value: u8 = first_register_value >> 1;

        cpu.set_v_register(0xF, value & 0x1);
        cpu.set_v_register(first_register_number, value);
    }

    fn put_value_of_inverted_subtraction_operation_between_v_registers_on_first_passed_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_inverted_subtraction_operation_between_v_registers_on_first_passed_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);
        let second_register_value: u8 = cpu.get_v_register(second_register_number);

        let (value, flag): (u8, bool) = second_register_value.overflowing_sub(first_register_value);

        cpu.set_v_register(0xF, !flag as u8);
        cpu.set_v_register(first_register_number, value);
    }

    fn put_value_of_bitwise_shift_left_operation_between_v_registers_on_first_passed_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_bitwise_shift_left_operation_between_v_registers_on_first_passed_register");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);

        let value: u8 = first_register_value << 1;

        cpu.set_v_register(0xF, (value >> 7) & 0x1);
        cpu.set_v_register(first_register_number, value);
    }

    fn skip_next_instruction_if_v_registers_values_are_different(&self, cpu: &mut Cpu) {
        println!("skip_next_instruction_if_v_registers_values_are_different");

        let first_register_number: usize = (self.opcode as usize) >> 8 & 0x000F;
        let second_register_number: usize = (self.opcode as usize) >> 4 & 0x000F;

        let first_register_value: u8 = cpu.get_v_register(first_register_number);
        let second_register_value: u8 = cpu.get_v_register(second_register_number);

        if first_register_value != second_register_value {
            cpu.increase_program_counter(0x2);
        }
    }

    fn put_argument_value_on_i_register(&self, cpu: &mut Cpu) {
        println!("put_argument_value_on_i_register");

        let argument: u16 = self.opcode & 0xFFF;

        cpu.set_i_register(argument);
    }

    fn jump_to_argument_value_plus_v0(&self, cpu: &mut Cpu) {
        println!("put_argument_value_on_i_register");

        let argument: u16 = self.opcode & 0xFFF;
        let address: usize = (argument as usize) + (cpu.get_v_register(0x0) as usize);

        cpu.set_program_counter(address);
    }

    fn put_value_of_bitwise_or_operation_between_argument_and_random_byte_on_passed_v_register(&self, cpu: &mut Cpu) {
        println!("put_value_of_bitwise_or_operation_between_argument_and_random_byte_on_passed_v_register");

        let register_number: usize = ((self.opcode as usize) >> 8) & 0xF;
        let argument: u8 = (self.opcode as u8) & 0xFF;
        let random_byte: u8 = rand::thread_rng().gen_range(0x0..0xFF);

        cpu.set_v_register(
            register_number,
            argument | random_byte
        );
    }

    fn draw_byte_sprite_starting_at_location_i_on_register_stored_location(&self, cpu: &mut Cpu) {
        println!("draw_byte_sprite_starting_at_location_i_on_register_stored_location");
    }

    fn skip_next_instruction_if_key_with_v_register_value_is_pressed(&self, cpu: &mut Cpu) {
        println!("skip_next_instruction_if_key_with_v_register_value_is_pressed");
    }

    fn skip_next_instruction_if_key_with_v_register_value_is_not_pressed(&self, cpu: &mut Cpu) {
        println!("skip_next_instruction_if_key_with_v_register_value_is_not_pressed");
    }

    fn put_delay_timer_value_on_v_register(&self, cpu: &mut Cpu) {
        println!("put_delay_timer_value_on_v_register");

        let register_number: usize = (self.opcode as usize) >> 8 & 0xF;

        cpu.set_v_register(
            register_number, 
            cpu.get_delay_timer()
        );
    }

    fn wait_for_key_press_and_store_value_on_v_register(&self, cpu: &mut Cpu) {
        println!("wait_for_key_press_and_store_value_on_v_register");
    }

    fn put_v_register_value_on_delay_timer(&self, cpu: &mut Cpu) {
        println!("put_v_register_value_on_delay_timer");

        let register_number: usize = (self.opcode as usize) >> 8 & 0xF;

        cpu.set_delay_timer(
            cpu.get_v_register(register_number)
        );
    }

    fn put_v_register_value_on_sound_timer(&self, cpu: &mut Cpu) {
        println!("put_v_register_value_on_sound_timer");

        let register_number: usize = (self.opcode as usize) >> 8 & 0xF;

        cpu.set_sound_timer(
            cpu.get_v_register(register_number)
        );
    }

    fn add_v_register_value_to_i_register_value(&self, cpu: &mut Cpu) {
        println!("add_v_register_value_to_i_register_value");

        let register_number: usize = (self.opcode as usize) >> 8 & 0xF;

        let register_value: u16 = cpu.get_v_register(register_number) as u16;

        cpu.set_i_register(
            cpu.get_i_register().wrapping_add(register_value)
        );
    }

    fn put_location_of_sprite_for_v_register_digit_on_i_register(&self, cpu: &mut Cpu) {
        println!("put_location_of_sprite_for_v_register_digit_on_i_register");
    }

    fn put_bcd_representation_of_v_register_in_memory_locations_starting_on_i_register_location(
        &self, 
        cpu: &mut Cpu,
        memory: &mut Memory,
    ) {
        println!("put_bcd_representation_of_v_register_in_memory_locations_starting_on_i_register_location");

        let register_number: usize = (self.opcode as usize) >> 8 & 0xF;

        let register_value: u8 = cpu.get_v_register(register_number);

        let mut memory_location: usize = cpu.get_i_register() as usize;

        let hundreds: u8 = register_value.div(100);
        let tens: u8 = register_value.sub(hundreds * 100).div(10);
        let ones: u8 = register_value.sub(hundreds * 100).sub(tens * 10);

        memory.set(
            memory_location, 
            hundreds as u8
        );

        memory_location += 1;

        memory.set(
            memory_location, 
            tens as u8
        );

        memory_location += 1;

        memory.set(
            memory_location, 
            ones as u8
        );
    }

    fn put_values_of_v_registers_from_v0_to_passed_v_register_in_memory_starting_on_i_register_location(
        &self,
        cpu: &mut Cpu,
        memory: &mut Memory,
    ) {
        println!("put_values_of_v_registers_from_v0_to_passed_v_register_in_memory_starting_on_i_register_location");

        let final_register_number: usize = (self.opcode as usize) >> 8 & 0xF;

        let mut memory_location: usize = cpu.get_i_register() as usize;

        for register_number in 0..=final_register_number {
            let register_value: u8 = cpu.get_v_register(register_number);

            memory.set(memory_location, register_value);

            memory_location += 1;
        }
    }

    fn put_values_on_v_registers_from_v0_to_passed_v_register_from_memory_starting_on_i_register_location(
        &self,
        cpu: &mut Cpu,
        memory: &mut Memory,
    ) {
        println!("put_values_on_v_registers_from_v0_to_passed_v_register_from_memory_starting_on_i_register_location");

        let final_register_number: usize = (self.opcode as usize) >> 8 & 0xF;

        let mut memory_location: usize = cpu.get_i_register() as usize;

        for register_number in 0..=final_register_number {
            let memory_value: u8 = memory.get(memory_location);

            cpu.set_v_register(register_number, memory_value);

            memory_location += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use test_case::test_case;
    use super::*;

    #[test]
    fn it_should_initialize_the_instruction() {
        let instruction: Instruction = Instruction::initialize(0xCA, 0xFE);

        assert_eq!(0xCAFE, instruction.opcode);
    }

    #[test]
    fn it_should_clear_the_display() {
        let mut instruction: Instruction = Instruction::initialize(0x00, 0xE0);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        interface.expect_set_buffer()
            .with(
                eq(vec!(Interface::BLACK; Interface::WIDTH * Interface::HEIGHT))
            )
            .returning(|_| ());

        instruction.interpret(&mut cpu, &mut memory, &mut interface);
    }

    #[test]
    fn it_should_return_from_subroutine() {
        let mut instruction: Instruction = Instruction::initialize(0x00, 0xEE);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.stack_push(0xCAF);

        assert_eq!(0x200, cpu.get_program_counter());

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xCAF, cpu.get_program_counter());
    }

    #[test]
    fn it_should_jump_to_address() {
        let mut instruction: Instruction = Instruction::initialize(0x1C, 0xAF);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xCAF, cpu.get_program_counter());
    }

    #[test]
    fn it_should_call_subroutine_at_address() {
        let mut instruction: Instruction = Instruction::initialize(0x2C, 0xAF);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xCAF, cpu.get_program_counter());
        assert_eq!(0x200, cpu.stack_pop());
    }

    #[test_case(0xFF, 0xAA, 0x200 ; "with different values")]
    #[test_case(0xFF, 0xFF, 0x202 ; "with the same value")]
    fn it_should_skip_next_instruction_if_argument_equals_v_register_value(
        instruction_value: u8, 
        register_value: u8, 
        program_counter: usize
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x3A, instruction_value);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(program_counter, cpu.get_program_counter());
    }

    #[test_case(0xFF, 0xAA, 0x202 ; "with different values")]
    #[test_case(0xFF, 0xFF, 0x200 ; "with the same value")]
    fn it_should_skip_next_instruction_if_argument_differs_v_register_value(
        instruction_value: u8, 
        register_value: u8, 
        program_counter: usize
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x4A, instruction_value);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(program_counter, cpu.get_program_counter());
    }

    #[test_case(0xFF, 0xAA, 0x200 ; "with different values")]
    #[test_case(0xFF, 0xFF, 0x202 ; "with the same value")]
    fn it_should_skip_next_instruction_if_v_registers_values_are_the_same(
        first_register_value: u8, 
        second_register_value: u8, 
        program_counter: usize
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x5A, 0xB0);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, first_register_value);
        cpu.set_v_register(0xB, second_register_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(program_counter, cpu.get_program_counter());
    }

    #[test]
    fn it_should_put_argument_value_on_v_register() {
        let mut instruction: Instruction = Instruction::initialize(0x6A, 0xCA);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        assert_eq!(0x00, cpu.get_v_register(0xA));

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xCA, cpu.get_v_register(0xA));
    }

    #[test_case(0x03, 0x07, 0xA ; "without overflow")]
    #[test_case(0xFF, 0x01, 0x00 ; "with overflow")]
    fn it_should_add_argument_value_to_v_register(
        register_value: u8, 
        argument_value: u8, 
        result: u8
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x7A, argument_value);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(result, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_put_v_register_value_on_other_v_register() {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC0);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xC, 0xFE);

        assert_eq!(0x00, cpu.get_v_register(0xA));

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xFE, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_put_value_of_bitwise_or_operation_between_v_registers_on_first_passed_register() {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC1);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, 0b10101010);
        cpu.set_v_register(0xC, 0b11110000);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0b11111010, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_put_value_of_bitwise_and_operation_between_v_registers_on_first_passed_register() {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC2);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, 0b10101010);
        cpu.set_v_register(0xC, 0b11110000);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0b10100000, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_put_value_of_bitwise_xor_operation_between_v_registers_on_first_passed_register() {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC3);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, 0b10101010);
        cpu.set_v_register(0xC, 0b11110000);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0b01011010, cpu.get_v_register(0xA));
    }

    #[test_case(0x03, 0x07, 0xA, 0x0 ; "without overflow")]
    #[test_case(0xFF, 0x01, 0x00, 0x1 ; "with overflow")]
    fn it_should_put_value_of_sum_operation_between_v_registers_on_first_passed_register(
        first_value: u8, 
        second_value: u8, 
        result: u8,
        flag: u8,
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC4);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, first_value);
        cpu.set_v_register(0xC, second_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(result, cpu.get_v_register(0xA));
        assert_eq!(flag, cpu.get_v_register(0xF));
    }

    #[test_case(0x07, 0x03, 0x4, 0x1 ; "without underflow")]
    #[test_case(0x00, 0x01, 0xFF, 0x0 ; "with underflow")]
    fn it_should_put_value_of_subtraction_operation_between_v_registers_on_first_passed_register(
        first_value: u8, 
        second_value: u8, 
        result: u8,
        flag: u8,
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC5);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, first_value);
        cpu.set_v_register(0xC, second_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(result, cpu.get_v_register(0xA));
        assert_eq!(flag, cpu.get_v_register(0xF));
    }

    #[test_case(0b01010101, 0b00101010, 0x0 ; "with 0 on least significant bit")]
    #[test_case(0b10101010, 0b01010101, 0x1 ; "with 1 on least significant bit")]
    fn it_should_put_value_of_bitwise_shift_right_operation_between_v_registers_on_first_passed_register(
        register_value: u8, 
        result: u8,
        flag: u8,
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC6);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(result, cpu.get_v_register(0xA));
        assert_eq!(flag, cpu.get_v_register(0xF));
    }

    #[test_case(0x03, 0x07, 0x4, 0x1 ; "without underflow")]
    #[test_case(0x01, 0x00, 0xFF, 0x0 ; "with underflow")]
    fn it_should_put_value_of_inverted_subtraction_operation_between_v_registers_on_first_passed_register(
        first_value: u8, 
        second_value: u8, 
        result: u8,
        flag: u8,
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC7);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, first_value);
        cpu.set_v_register(0xC, second_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(result, cpu.get_v_register(0xA));
        assert_eq!(flag, cpu.get_v_register(0xF));
    }

    #[test_case(0b10101010, 0b01010100, 0x0 ; "with 0 on most significant bit")]
    #[test_case(0b01010101, 0b10101010, 0x1 ; "with 1 on most significant bit")]
    fn it_should_put_value_of_bitwise_shift_left_operation_between_v_registers_on_first_passed_register(
        register_value: u8, 
        result: u8,
        flag: u8,
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xCE);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(result, cpu.get_v_register(0xA));
        assert_eq!(flag, cpu.get_v_register(0xF));
    }

    #[test_case(0xFF, 0xAA, 0x202 ; "with different values")]
    #[test_case(0xFF, 0xFF, 0x200 ; "with the same value")]
    fn it_should_skip_next_instruction_if_v_registers_values_are_different(
        first_register_value: u8, 
        second_register_value: u8, 
        program_counter: usize
    ) {
        let mut instruction: Instruction = Instruction::initialize(0x9A, 0xB0);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xA, first_register_value);
        cpu.set_v_register(0xB, second_register_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(program_counter, cpu.get_program_counter());
    }

    #[test]
    fn it_should_put_argument_value_on_i_register() {
        let mut instruction: Instruction = Instruction::initialize(0xAC, 0xAF);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0x0CAF, cpu.get_i_register());
    }

    #[test]
    fn it_should_jump_to_argument_value_plus_v0() {
        let mut instruction: Instruction = Instruction::initialize(0xB0, 0x03);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0x0, 0x4);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0x7, cpu.get_program_counter());
    }

    #[test]
    fn it_should_put_value_of_bitwise_or_operation_between_argument_and_random_byte_on_passed_v_register() {
        let mut instruction: Instruction = Instruction::initialize(0xCA, 0xFF);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        assert_eq!(0x0, cpu.get_v_register(0xA));

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xFF, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_draw_byte_sprite_starting_at_location_i_on_register_stored_location() {
        let instruction: Instruction = Instruction::initialize(0xDA, 0xCF);
    }

    #[test]
    fn it_should_skip_next_instruction_if_key_with_v_register_value_is_pressed() {
        let instruction: Instruction = Instruction::initialize(0xEA, 0x9E);
    }

    #[test]
    fn it_should_skip_next_instruction_if_key_with_v_register_value_is_not_pressed() {
        let instruction: Instruction = Instruction::initialize(0xEA, 0x41);
    }

    #[test]
    fn it_should_put_delay_timer_value_on_v_register() {
        let mut instruction: Instruction = Instruction::initialize(0xFC, 0x07);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_delay_timer(0xCA);

        assert_eq!(0x0, cpu.get_v_register(0xC));

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xCA, cpu.get_v_register(0xC));
    }

    #[test]
    fn it_should_wait_for_key_press_and_store_value_on_v_register() {
        let instruction: Instruction = Instruction::initialize(0xFC, 0x0A);
    }

    #[test]
    fn it_should_put_v_register_value_on_delay_timer() {
        let mut instruction: Instruction = Instruction::initialize(0xFC, 0x15);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xC, 0xFE);

        assert_eq!(0x0, cpu.get_delay_timer());

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xFE, cpu.get_delay_timer());
    }

    #[test]
    fn it_should_put_v_register_value_on_sound_timer() {
        let mut instruction: Instruction = Instruction::initialize(0xFC, 0x18);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0xC, 0xFE);

        assert_eq!(0x0, cpu.get_sound_timer());

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0xFE, cpu.get_sound_timer());
    }

    #[test_case(0x3, 0x4, 0x7 ; "without overflow")]
    #[test_case(0xFFFF, 0x1, 0x0 ; "with overflow")]
    fn it_should_add_v_register_value_to_i_register_value(
        i_register_value: u16,
        v_register_value: u8,
        result: u16
    ) {
        let mut instruction: Instruction = Instruction::initialize(0xFC, 0x1E);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_i_register(i_register_value);
        cpu.set_v_register(0xC, v_register_value);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(result, cpu.get_i_register());
    }

    #[test]
    fn it_should_put_location_of_sprite_for_v_register_digit_on_i_register() {
        let instruction: Instruction = Instruction::initialize(0xFC, 0x29);
    }

    #[test]
    fn it_should_put_bcd_representation_of_v_register_in_memory_locations_starting_on_i_register_location() {
        let mut instruction: Instruction = Instruction::initialize(0xFC, 0x33);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_i_register(0x200);
        cpu.set_v_register(0xC, 198);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(1, memory.get(0x200));
        assert_eq!(9, memory.get(0x201));
        assert_eq!(8, memory.get(0x202));
    }

    #[test]
    fn it_should_put_values_of_v_registers_from_v0_to_passed_v_register_in_memory_starting_on_i_register_location() {
        let mut instruction: Instruction = Instruction::initialize(0xFF, 0x55);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        cpu.set_v_register(0x0, 0x10);
        cpu.set_v_register(0x1, 0x11);
        cpu.set_v_register(0x2, 0x12);
        cpu.set_v_register(0x3, 0x13);
        cpu.set_v_register(0x4, 0x14);
        cpu.set_v_register(0x5, 0x15);
        cpu.set_v_register(0x6, 0x16);
        cpu.set_v_register(0x7, 0x17);
        cpu.set_v_register(0x8, 0x18);
        cpu.set_v_register(0x9, 0x19);
        cpu.set_v_register(0xA, 0x1A);
        cpu.set_v_register(0xB, 0x1B);
        cpu.set_v_register(0xC, 0x1C);
        cpu.set_v_register(0xD, 0x1D);
        cpu.set_v_register(0xE, 0x1E);
        cpu.set_v_register(0xF, 0x1F);

        cpu.set_i_register(0x200);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0x10, memory.get(0x200));
        assert_eq!(0x11, memory.get(0x201));
        assert_eq!(0x12, memory.get(0x202));
        assert_eq!(0x13, memory.get(0x203));
        assert_eq!(0x14, memory.get(0x204));
        assert_eq!(0x15, memory.get(0x205));
        assert_eq!(0x16, memory.get(0x206));
        assert_eq!(0x17, memory.get(0x207));
        assert_eq!(0x18, memory.get(0x208));
        assert_eq!(0x19, memory.get(0x209));
        assert_eq!(0x1A, memory.get(0x20A));
        assert_eq!(0x1B, memory.get(0x20B));
        assert_eq!(0x1C, memory.get(0x20C));
        assert_eq!(0x1D, memory.get(0x20D));
        assert_eq!(0x1E, memory.get(0x20E));
        assert_eq!(0x1F, memory.get(0x20F));
    }

    #[test]
    fn it_should_put_values_on_v_registers_from_v0_to_passed_v_register_from_memory_starting_on_i_register_location() {
        let mut instruction: Instruction = Instruction::initialize(0xFF, 0x65);
        let mut cpu: Cpu = Cpu::initialize();
        let mut memory: Memory = Memory::initialize();
        let mut interface= Interface::default();

        memory.set(0x200, 0x10);
        memory.set(0x201, 0x11);
        memory.set(0x202, 0x12);
        memory.set(0x203, 0x13);
        memory.set(0x204, 0x14);
        memory.set(0x205, 0x15);
        memory.set(0x206, 0x16);
        memory.set(0x207, 0x17);
        memory.set(0x208, 0x18);
        memory.set(0x209, 0x19);
        memory.set(0x20A, 0x1A);
        memory.set(0x20B, 0x1B);
        memory.set(0x20C, 0x1C);
        memory.set(0x20D, 0x1D);
        memory.set(0x20E, 0x1E);
        memory.set(0x20F, 0x1F);

        cpu.set_i_register(0x200);

        instruction.interpret(&mut cpu, &mut memory, &mut interface);

        assert_eq!(0x10, cpu.get_v_register(0x0));
        assert_eq!(0x11, cpu.get_v_register(0x1));
        assert_eq!(0x12, cpu.get_v_register(0x2));
        assert_eq!(0x13, cpu.get_v_register(0x3));
        assert_eq!(0x14, cpu.get_v_register(0x4));
        assert_eq!(0x15, cpu.get_v_register(0x5));
        assert_eq!(0x16, cpu.get_v_register(0x6));
        assert_eq!(0x17, cpu.get_v_register(0x7));
        assert_eq!(0x18, cpu.get_v_register(0x8));
        assert_eq!(0x19, cpu.get_v_register(0x9));
        assert_eq!(0x1A, cpu.get_v_register(0xA));
        assert_eq!(0x1B, cpu.get_v_register(0xB));
        assert_eq!(0x1C, cpu.get_v_register(0xC));
        assert_eq!(0x1D, cpu.get_v_register(0xD));
        assert_eq!(0x1E, cpu.get_v_register(0xE));
        assert_eq!(0x1F, cpu.get_v_register(0xF));
    }
}