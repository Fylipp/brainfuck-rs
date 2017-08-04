use ::std;
use ::std::option::Option;
use ::std::io::Read;
use super::constants::*;

#[cfg(test)]
mod tests;

/// The default input
pub fn stdin() -> u8 {
    match std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok()) {
        Some(data) => data,
        None => panic!("Received no input")
    }
}

/// The default output
pub fn stdout(out: u8) {
    print!("{}", out as char);
}

/// Represents a Brainfuck machine's state
pub struct BrainfuckMachine<'a> {
    input: fn() -> u8,
    output: fn(u8),
    ptr: u16,
    cells: [u8; std::u16::MAX as usize],
    instr: &'a [u8],
    ip: usize,
    stack: Vec<usize>,
    done: bool
}

impl<'a> BrainfuckMachine<'a> {
    /// Creates a new instance with the given instructions and custom I/O
    pub fn new(instr: &'a [u8], input: fn() -> u8, output: fn(u8)) -> BrainfuckMachine {
        return BrainfuckMachine {
            input: input,
            output: output,
            ptr: 0,
            cells: [0; std::u16::MAX as usize],
            instr: instr,
            ip: 0,
            stack: Vec::with_capacity(16),
            done: false
        };
    }

    /// Creates a new instance with the standard I/O functions
    pub fn new_stdio(instr: &'a [u8]) -> BrainfuckMachine {
        return BrainfuckMachine::new(instr, stdin, stdout);
    }

    /// Creates a new instance with a dummy I/O implementation (input always 0 and output ignored)
    pub fn new_dummy(instr: &'a [u8]) -> BrainfuckMachine {
        return BrainfuckMachine::new(instr, || 0, |_| {});
    }

    /// Execute the next instruction and return whether the machine is done
    pub fn execute_next(&mut self) -> bool {
        if self.ip >= self.instr.len() {
            self.done = true;
        } else {
            match self.instr[self.ip] {
                INCR_PTR => self.ptr = self.ptr.wrapping_add(1),
                DECR_PTR => self.ptr = self.ptr.wrapping_sub(1),
                INCR_VAL => self.cells[self.ptr as usize] = self.cells[self.ptr as usize].wrapping_add(1),
                DECR_VAL => self.cells[self.ptr as usize] = self.cells[self.ptr as usize].wrapping_sub(1),
                LOOP_START => {
                    if self.cells[self.ptr as usize] == 0 {
                        let mut closing: Option<usize> = std::option::Option::None;

                        let mut levels: u32 = 0;

                        for i in (std::ops::RangeInclusive {
                            start: self.ip + 1,
                            end: self.instr.len() - 1
                        }) {
                            match self.instr[i] {
                                LOOP_START => levels += 1,
                                LOOP_END => {
                                    if levels == 0 {
                                        closing = Option::Some(i);
                                        break;
                                    } else {
                                        levels -= 1;
                                    }
                                },
                                _ => {}
                            }
                        }
                        match closing {
                            None => panic!("Loop starting at {} has no end", self.ip),
                            Some(p) => self.ip = p
                        }
                    } else {
                        self.stack.push(self.ip);
                    }
                }
                LOOP_END => match self.stack.pop() {
                    Some(ip) => {
                        if self.cells[self.ptr as usize] != 0 {
                            self.ip = ip.wrapping_sub(1);
                        }
                    }
                    None => panic!("Loop ending at {} has no beginning", self.ip)
                },
                OUTPUT => (self.output)(self.cells[self.ptr as usize]),
                INPUT => self.cells[self.ptr as usize] = (self.input)(),
                _ => {}
            }

            self.ip += 1;
        }

        return self.done;
    }

    /// Executes all (remaining) instructions
    pub fn execute_all(&mut self) {
        while !self.execute_next() {}
    }
}
