use std::{collections::VecDeque, io::stdin};

pub struct Input {
    queue: VecDeque<u8>
}

impl Input {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new()
        }
    }

    fn read(&mut self) {
        let mut input_line = String::new();
        stdin().read_line(&mut input_line).expect("Failed to read line");

        for c in input_line.chars() {
            if c != '\n' {
                self.queue.push_back(c as u8);
            }
        }
    }

    pub fn next(&mut self) -> u8 {
        while self.queue.is_empty() {
            self.read();
        }

        self.queue.pop_front().unwrap()
    }
}