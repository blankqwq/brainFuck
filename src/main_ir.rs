use std::io::{Read, Write};
use brain_fuck::ir::{Code, IR};
use brain_fuck::opcode;

#[derive(Debug)]
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
        let code = opcode::Code::from(data)?;
        let code = Code::from(code.instrs)?;
        let code_len = code.instrs.len();
        let mut sp = 0;
        let mut pc = 0;
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                IR::SHR(x) => {
                    sp += x as usize;
                    let len = self.stack.len();
                    if sp >= len {
                        let expand = sp - len;
                        for _ in 0..=expand {
                            self.stack.push(0)
                        }
                    }
                }
                IR::SHL(x) => {
                    for _ in 0..x {
                        if sp != 0 {
                            sp -= 1
                        } else {
                            break;
                        }
                    }
                }
                IR::ADD(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_add(x).0;
                }
                IR::SUB(x) => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(x).0;
                }
                IR::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                }
                IR::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0;1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                }
                IR::JIZ(x) => {
                    if self.stack[sp] == 0x00 {
                        pc = x as usize
                    }
                }
                IR::JNE(x) => {
                    if self.stack[sp] != 0x00 {
                        pc = x as usize
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
    //
    let mut interpreter = Interpreter::new();
    interpreter.run(file_data)?;
    Ok(())
}