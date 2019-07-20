use std::convert::TryInto;

pub struct VM {
    pc: u32,
    mar: u32,
    stack: Vec<i32>,
    return_stack: Vec<u32>,
    memory: Vec<i32>,
    state: State,
}

#[derive(PartialEq)]
enum State {
    Running,
    Halt
}

impl VM {
    pub fn new() -> VM {
        VM {
            pc: 0,
            mar: 0,
            stack: Vec::new(),
            return_stack: Vec::new(),
            memory: vec![0; 2048],
            state: State::Running
        }
    }

    pub fn load(&mut self, program: &Vec<u32>) {
        let mut i = 0;
        for n in program.iter() {
            self.memory[i] = (*n) as i32;
            i += 1;
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.state == State::Halt {
                break;
            }

            let cell: u32 = self.memory[self.pc as usize] as u32;
            self.pc += 1;

            let instruction = (cell & 0xFF000000) >> 24;
            match instruction {
                // JMP
                0x10 => {
                    let address = cell & 0x00FFFFFF;
                    self.pc = address.try_into().unwrap();
                },
                // JMZ
                0x20 => {
                    let address = cell & 0x00FFFFFF;
                    let val = self.stack.pop().unwrap();
                    if val == 0 {
                        self.pc = address.try_into().unwrap();
                    }
                },
                // LOAD
                0x30 => {
                    let address = cell & 0x00FFFFFF;
                    self.mar = address.try_into().unwrap();
                    self.stack.push(self.memory[self.mar as usize] as i32);
                },
                // LOADC
                0x40 => {
                    let value = cell & 0x00FFFFFF;
                    self.stack.push(value as i32);
                },
                // STORE
                0x50 => {
                    let address = cell & 0x00FFFFFF;
                    self.mar = address.try_into().unwrap();
                    let value = self.stack.pop().unwrap();
                    self.memory[self.mar as usize] = value;
                },
                // CALL
                0x60 => {
                    let address = cell & 0x00FFFFFF;
                    self.return_stack.push(self.pc);
                    self.pc = address.try_into().unwrap();
                },
                // WRITE
                0x70 => {
                    let value = self.stack.pop().unwrap();
                    println!("{}", value);
                },
                // ADD
                0x80 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a + b);
                },
                // SUB
                0x90 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b - a);
                },
                // DIV
                0xA0 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(b / a);
                },
                // MUL
                0xB0 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a * b);
                },
                // ODD
                0xC0 => {
                    let a = self.stack.pop().unwrap();
                    if (a % 2) == 1 {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                // LT
                0xD0 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    if b < a {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                // LTE
                0xE0 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    if b <= a {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                // GT
                0xF0 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    if b > a {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                // GTE
                0xF1 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    if b >= a {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                // EQ
                0xF2 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    if b == a {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                // NEQ
                0xF3 => {
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    if a != b {
                        self.stack.push(1);
                    } else {
                        self.stack.push(0);
                    }
                },
                // NOOP
                0xF4 => {
                    continue;
                },
                // RET
                0xF5 => {
                    let address = self.return_stack.pop().unwrap();
                    self.pc = address;
                },
                // HALT
                0xF6 => {
                    self.state = State::Halt;
                },
                _ => (),
            }
        }
    }
}
