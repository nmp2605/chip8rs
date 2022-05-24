pub struct Memory {
    bytes: [u8; 0xFFF],
}

impl Memory {
    pub const FONT: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

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

    pub fn set_font(&mut self) {
        for (index, byte) in Memory::FONT.iter().cloned().enumerate() {
            self.bytes[0x50 + index] = byte;
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
    fn it_should_set_the_font() {
        let mut memory: Memory = Memory::initialize();

        memory.set_font();

        for (index, byte) in Memory::FONT.iter().cloned().enumerate() {
            assert_eq!(byte, memory.bytes[0x50 + index]);
        }
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