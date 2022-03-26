pub struct Memory {
    bytes: [u8; 0xFFF],
}

impl Memory {
    pub fn initialize() -> Memory {
        Memory {
            bytes: [0x0; 0xFFF],
        }
    }

    pub fn store_program(&mut self, program: Vec<u8>) {
        for (index, byte) in program.iter().cloned().enumerate() {
            self.bytes[0x200 + index] = byte;
        }
    }

    pub fn set(&mut self, location: usize, byte: u8) {
        self.bytes[location] = byte;
    }

    pub fn get(&self, location: usize) -> u8 {
        self.bytes[location]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_initialize_the_memory() {
        let memory: Memory = Memory::initialize();

        assert_eq!([0x0; 0xFFF], memory.bytes);
    }

    #[test]
    fn it_should_store_a_program() {
        let mut memory: Memory = Memory::initialize();

        let vector: Vec<u8> = vec![0x7, 0x6, 0x5];

        memory.store_program(vector);

        assert_eq!(0x7, memory.bytes[0x200]);
        assert_eq!(0x6, memory.bytes[0x201]);
        assert_eq!(0x5, memory.bytes[0x202]);
    }

    #[test]
    fn it_should_set_a_byte() {
        let mut memory: Memory = Memory::initialize();

        memory.set(0x0, 0x8);

        assert_eq!(0x8, memory.bytes[0x0]);
    }

    #[test]
    fn it_should_get_a_byte() {
        let mut memory: Memory = Memory::initialize();

        memory.bytes[0xF] = 0x7;

        assert_eq!(0x7, memory.get(0xF));
    }
}