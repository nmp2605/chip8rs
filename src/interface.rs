
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
    pub const WIDTH: usize = 640;
    pub const HEIGHT: usize = 320;
    pub const BLACK: u32 = 0;
    pub const WHITE: u32 = 16777215;

    pub fn initialize() -> Self {
        let window = Window::new(
            Interface::TITLE, 
            Interface::WIDTH, 
            Interface::HEIGHT, 
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

    pub fn get_buffer(&self) -> Vec<u32> {
        self.buffer.clone()
    }

    pub fn set_buffer(&mut self, buffer: Vec<u32>) {
        self.buffer = buffer.clone();

        self.window.update_with_buffer(&self.buffer, Interface::WIDTH, Interface::HEIGHT);
    }
}