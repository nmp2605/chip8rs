# Chip-8.rs

This project's main purpose is to write a readable, understandable interpreter for Chip-8 using Rust. It is meant for developers that either never coded in Rust and/or never worked on an interpreter or emulator.

What's Still Missing
--------------------

The interpreter is a work in progress. It are still some key parts missing, such as:

- Sound
- Enhance performance
- Debugging on some popular games

Requirements
------------

- [Rust](https://www.rust-lang.org/pt-BR/tools/install) >= 1.59.0

Running It Locally
------------------

To run a ROM in the interpreter, you need to pass the ROM location as an argument when calling the program. For instance, if you're using `cargo`, you can do it like this:

       cargo run ../../my-chip-8-roms/Pong.ch8

You can also run the tests in the usual Rust way:

       cargo test

Project Structure
-----------------

To make it feel "realistic," I've tried to separate everything as if it was a real computer:

- Motherboard: takes care of booting up the fetch-decode loop.
- CPU: fetches and decodes instructions, takes care of registers, timers, and the stack.
- Memory: stores and reads data from the ROM.
- Instruction: this was separated from the CPU because of the amount of data, and it localizes and executes the instructions on Chip-8.
- Interface: loads the screen where the emulator runs, controlling the display and input using [minifb](https://docs.rs/minifb/latest/minifb/).
- Keyboard: contains information about which keys will be used in the interpreter.

Finding ROMs
------------

To run the interpreter, you need to have ROMs that are compatible with Chip-8. These are some resources that can be found on Github:

- [Test ROM by corax89](https://github.com/corax89/chip8-test-rom)
- [Game and Program ROMs by kripod](https://github.com/kripod/chip8-roms)

Resources
---------

The main resources I've used when developing the interpreter are:

- [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Awesome Chip-8](https://chip-8.github.io/links/)
