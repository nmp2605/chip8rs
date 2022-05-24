
use minifb::{Window, WindowOptions};

#[cfg(test)]
use mockall::automock;

pub struct Interface {
    window: Window,
    buffer: Vec<u32>,
}

#[cfg_attr(test, automock)]
impl Interface {
    pub const TITLE: &'static str = "Chip-8";
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;
    pub const BLACK: u32 = 0;
    pub const WHITE: u32 = 16777215;

    pub fn initialize() -> Self {
        let window = Window::new(
            Interface::TITLE, 
            Interface::WIDTH * 10, 
            Interface::HEIGHT * 10, 
        WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        Interface {
            window: window,
            buffer: vec![0; Interface::WIDTH * Interface::HEIGHT]
        }
    }

    pub fn window_is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn refresh(&mut self) {
        self.window.update_with_buffer(&self.buffer, Interface::WIDTH, Interface::HEIGHT)
            .expect("The pixels on screen could not be updated.");
    }

    pub fn clear(&mut self) {
        self.buffer = vec!(Interface::BLACK; Interface::WIDTH * Interface::HEIGHT);
    }

    pub fn draw_pixel(&mut self, state: bool, x: usize, y: usize) -> bool {
        let index: usize = x + (y * Interface::WIDTH);

        let color: u32 = match state {
            true => Interface::WHITE,
            false => Interface::BLACK,
        };

        let flag: bool = self.buffer[index] == Interface::WHITE;

        self.buffer[index] = Interface::WHITE;

        flag
    }
}