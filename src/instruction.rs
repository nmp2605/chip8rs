use crate::cpu::Cpu;
use rand::Rng;

pub struct Instruction {
    opcode: u16,
}

impl Instruction {
    pub fn initialize(first_byte: u8, second_byte: u8) -> Instruction {
        Instruction {
            opcode: u16::from(first_byte) << 8 | u16::from(second_byte),
        }
    }

    pub fn interpret(&mut self, cpu: &mut Cpu) {
        match self.opcode {
            0x00E0 => self.clear_display(),
            0x00EE => self.return_from_subroutine(cpu),
            _ => (),
        }

        match self.opcode & 0xF000 {
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
            _ => (),
        }
    }

    fn clear_display(&self) {
        println!("clear_display");
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
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test]
    fn it_should_initialize_the_instruction() {
        let instruction: Instruction = Instruction::initialize(0xCA, 0xFE);

        assert_eq!(0xCAFE, instruction.opcode);
    }

    #[test]
    fn it_should_clear_the_display() {
        let instruction: Instruction = Instruction::initialize(0x00, 0xE0);
    }

    #[test]
    fn it_should_return_from_subroutine() {
        let mut instruction: Instruction = Instruction::initialize(0x00, 0xEE);
        let mut cpu: Cpu = Cpu::initialize();

        cpu.stack_push(0xCAF);

        assert_eq!(0x200, cpu.get_program_counter());

        instruction.interpret(&mut cpu);

        assert_eq!(0xCAF, cpu.get_program_counter());
    }

    #[test]
    fn it_should_jump_to_address() {
        let mut instruction: Instruction = Instruction::initialize(0x1C, 0xAF);
        let mut cpu: Cpu = Cpu::initialize();

        instruction.interpret(&mut cpu);

        assert_eq!(0xCAF, cpu.get_program_counter());
    }

    #[test]
    fn it_should_call_subroutine_at_address() {
        let mut instruction: Instruction = Instruction::initialize(0x2C, 0xAF);
        let mut cpu: Cpu = Cpu::initialize();

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, first_register_value);
        cpu.set_v_register(0xB, second_register_value);

        instruction.interpret(&mut cpu);

        assert_eq!(program_counter, cpu.get_program_counter());
    }

    #[test]
    fn it_should_put_argument_value_on_v_register() {
        let mut instruction: Instruction = Instruction::initialize(0x6A, 0xCA);
        let mut cpu: Cpu = Cpu::initialize();

        assert_eq!(0x00, cpu.get_v_register(0xA));

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu);

        assert_eq!(result, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_put_v_register_value_on_other_v_register() {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC0);
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_v_register(0xC, 0xFE);

        assert_eq!(0x00, cpu.get_v_register(0xA));

        instruction.interpret(&mut cpu);

        assert_eq!(0xFE, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_put_value_of_bitwise_or_operation_between_v_registers_on_first_passed_register() {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC1);
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_v_register(0xA, 0b10101010);
        cpu.set_v_register(0xC, 0b11110000);

        instruction.interpret(&mut cpu);

        assert_eq!(0b11111010, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_put_value_of_bitwise_and_operation_between_v_registers_on_first_passed_register() {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC2);
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_v_register(0xA, 0b10101010);
        cpu.set_v_register(0xC, 0b11110000);

        instruction.interpret(&mut cpu);

        assert_eq!(0b10100000, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_put_value_of_bitwise_xor_operation_between_v_registers_on_first_passed_register() {
        let mut instruction: Instruction = Instruction::initialize(0x8A, 0xC3);
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_v_register(0xA, 0b10101010);
        cpu.set_v_register(0xC, 0b11110000);

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, first_value);
        cpu.set_v_register(0xC, second_value);

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, first_value);
        cpu.set_v_register(0xC, second_value);

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, first_value);
        cpu.set_v_register(0xC, second_value);

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, register_value);

        instruction.interpret(&mut cpu);

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

        cpu.set_v_register(0xA, first_register_value);
        cpu.set_v_register(0xB, second_register_value);

        instruction.interpret(&mut cpu);

        assert_eq!(program_counter, cpu.get_program_counter());
    }

    #[test]
    fn it_should_put_argument_value_on_i_register() {
        let mut instruction: Instruction = Instruction::initialize(0xAC, 0xAF);
        let mut cpu: Cpu = Cpu::initialize();

        instruction.interpret(&mut cpu);

        assert_eq!(0x0CAF, cpu.get_i_register());
    }

    #[test]
    fn it_should_jump_to_argument_value_plus_v0() {
        let mut instruction: Instruction = Instruction::initialize(0xB0, 0x03);
        let mut cpu: Cpu = Cpu::initialize();

        cpu.set_v_register(0x0, 0x4);

        instruction.interpret(&mut cpu);

        assert_eq!(0x7, cpu.get_program_counter());
    }

    #[test]
    fn it_should_put_value_of_bitwise_or_operation_between_argument_and_random_byte_on_passed_v_register() {
        let mut instruction: Instruction = Instruction::initialize(0xCA, 0xFF);
        let mut cpu: Cpu = Cpu::initialize();

        assert_eq!(0x0, cpu.get_v_register(0xA));

        instruction.interpret(&mut cpu);

        assert_eq!(0xFF, cpu.get_v_register(0xA));
    }

    #[test]
    fn it_should_draw_byte_sprite_starting_at_location_i_on_register_stored_location() {
        let instruction: Instruction = Instruction::initialize(0xDA, 0xCF);
    }
}