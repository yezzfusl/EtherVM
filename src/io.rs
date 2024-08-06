// src/io.rs

use std::io::{self, Write};

pub trait IODevice {
    fn input(&mut self) -> u32;
    fn output(&mut self, value: u32);
}

pub struct IOController {
    devices: Vec<Box<dyn IODevice>>,
}

impl IOController {
    pub fn new() -> Self {
        IOController {
            devices: vec![Box::new(ConsoleDevice {})],
        }
    }

    pub fn input(&mut self) -> u32 {
        self.devices[0].input()
    }

    pub fn output(&mut self, value: u32) {
        self.devices[0].output(value);
    }
}

struct ConsoleDevice {}

impl IODevice for ConsoleDevice {
    fn input(&mut self) -> u32 {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap_or(0)
    }

    fn output(&mut self, value: u32) {
        println!("Output: {}", value);
    }
}

#[cfg(test)]
pub struct MockIOController {
    next_input: u32,
    last_output: u32,
}

#[cfg(test)]
impl MockIOController {
    pub fn new() -> Self {
        MockIOController {
            next_input: 0,
            last_output: 0,
        }
    }

    pub fn set_next_input(&mut self, value: u32) {
        self.next_input = value;
    }

    pub fn get_last_output(&self) -> u32 {
        self.last_output
    }
}

#[cfg(test)]
impl IODevice for MockIOController {
    fn input(&mut self) -> u32 {
        self.next_input
    }

    fn output(&mut self, value: u32) {
        self.last_output = value;
    }
}
