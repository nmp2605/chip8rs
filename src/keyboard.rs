use minifb::Key;

pub struct Keyboard {
    keys: [Key; 16],
}

impl Keyboard {
    pub fn initialize() -> Self {
        Keyboard {
            keys: [
                Key::X,
                Key::Key1,
                Key::Key2,
                Key::Key3,
                Key::Q,
                Key::W,
                Key::E,
                Key::A,
                Key::S,
                Key::D,
                Key::Z,
                Key::C,
                Key::Key4,
                Key::R,
                Key::F,
                Key::V,
            ],
        }
    }

    pub fn get_key(&self, index: usize) -> Key {
        self.keys[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_initialize_the_keyboard() {
        let keyboard: Keyboard = Keyboard::initialize();

        assert_eq!(keyboard.keys[0x0], Key::X);
        assert_eq!(keyboard.keys[0x1], Key::Key1);
        assert_eq!(keyboard.keys[0x2], Key::Key2);
        assert_eq!(keyboard.keys[0x3], Key::Key3);
        assert_eq!(keyboard.keys[0x4], Key::Q);
        assert_eq!(keyboard.keys[0x5], Key::W);
        assert_eq!(keyboard.keys[0x6], Key::E);
        assert_eq!(keyboard.keys[0x7], Key::A);
        assert_eq!(keyboard.keys[0x8], Key::S);
        assert_eq!(keyboard.keys[0x9], Key::D);
        assert_eq!(keyboard.keys[0xA], Key::Z);
        assert_eq!(keyboard.keys[0xB], Key::C);
        assert_eq!(keyboard.keys[0xC], Key::Key4);
        assert_eq!(keyboard.keys[0xD], Key::R);
        assert_eq!(keyboard.keys[0xE], Key::F);
        assert_eq!(keyboard.keys[0xF], Key::V);
    }

    #[test]
    fn it_should_get_a_key() {
        let keyboard: Keyboard = Keyboard::initialize();

        assert_eq!(keyboard.get_key(0x0), Key::X);
        assert_eq!(keyboard.get_key(0x1), Key::Key1);
        assert_eq!(keyboard.get_key(0x2), Key::Key2);
        assert_eq!(keyboard.get_key(0x3), Key::Key3);
        assert_eq!(keyboard.get_key(0x4), Key::Q);
        assert_eq!(keyboard.get_key(0x5), Key::W);
        assert_eq!(keyboard.get_key(0x6), Key::E);
        assert_eq!(keyboard.get_key(0x7), Key::A);
        assert_eq!(keyboard.get_key(0x8), Key::S);
        assert_eq!(keyboard.get_key(0x9), Key::D);
        assert_eq!(keyboard.get_key(0xA), Key::Z);
        assert_eq!(keyboard.get_key(0xB), Key::C);
        assert_eq!(keyboard.get_key(0xC), Key::Key4);
        assert_eq!(keyboard.get_key(0xD), Key::R);
        assert_eq!(keyboard.get_key(0xE), Key::F);
        assert_eq!(keyboard.get_key(0xF), Key::V);
    }
}