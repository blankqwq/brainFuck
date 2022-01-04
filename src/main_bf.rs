use std::io::{Read, Write};
use brain_fuck::opcode::{Code, OpCode};

struct Interpreter {
    stack: Vec<u8>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            stack: vec![0;1]
        }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();
        let mut sp = 0;
        let mut pc = 0;
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                OpCode::SHR => {
                    sp += 1;
                    if sp == self.stack.len() {
                        self.stack.push(0)
                    }
                }
                OpCode::SHL => {
                    if sp != 0 {
                        sp -= 1;
                    }
                }
                OpCode::ADD => {
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                }
                OpCode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                }
                OpCode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                }
                OpCode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0;1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                OpCode::LB => {
                    if self.stack[sp] == 0x00 {
                        pc = code.jtables[&pc]
                    }
                }
                OpCode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jtables[&pc]
                    }
                }
            }
            pc += 1
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let file_data: Vec<u8> = std::fs::read(&args[1])?;
    let mut interpreter = Interpreter::new();
    interpreter.run(file_data)?;
    Ok(())
}
