
use std::collections::HashSet;

use crate::keyboard::Keyboard;
use minifb::{Window, WindowOptions, KeyRepeat, Key};

#[cfg(test)]
use mockall::automock;

pub struct Interface {
    window: Window,
    buffer: Vec<u32>,
    keyboard: Keyboard,
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
            buffer: vec![0; Interface::WIDTH * Interface::HEIGHT],
            keyboard: Keyboard::initialize(),
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

        let old_state: bool = self.buffer[index] == Interface::WHITE;

        let color: u32 = match old_state != state {
            true => Interface::WHITE,
            false => Interface::BLACK,
        };

        self.buffer[index] = color;

        old_state == state
    }

    pub fn is_pressed(&self, key_code: usize) -> bool {
        self.window.is_key_down(
            self.keyboard.get_key(key_code)
        )
    }

    pub fn is_not_pressed(&self, key_code: usize) -> bool {
        self.is_pressed(key_code) == false
    }

    pub fn get_pressed_key(&self) -> Option<usize> {
        for (index, _key) in self.keyboard.get_keys().iter().enumerate() {
            if self.is_pressed(index) {
                return Some(index);
            }
        }

        return None;
    }
}