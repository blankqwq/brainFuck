use crate::opcode::OpCode;

#[derive(Debug)]
pub enum IR {
    SHR(u32),
    SHL(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32),
    JNE(u32),
}

pub struct Code {
    pub instrs: Vec<IR>,
}

impl Code {
    pub fn from(data: Vec<OpCode>) -> Result<Self, Box<dyn std::error::Error>> {
        // 缩减代码长度，更加高效的执行，减少match次数
        let mut irs: Vec<IR> = Vec::new();
        let mut jsp: Vec<u32> = Vec::new();
        for i in data.iter() {
            match i {
                OpCode::SHR => {
                    match irs.last_mut() {
                        Some(IR::SHR(x)) => {
                            *x += 1;
                        }
                        _ => {
                            irs.push(IR::SHR(1));
                        }
                    }
                }
                OpCode::SHL => {
                    match irs.last_mut() {
                        Some(IR::SHL(x)) => {
                            *x += 1;
                        }
                        _ => {
                            irs.push(IR::SHL(1));
                        }
                    }
                }
                OpCode::ADD => {
                    match irs.last_mut() {
                        Some(IR::ADD(x)) => {
                            let (b, _) = x.overflowing_add(1);
                            *x = b;
                        }
                        _ => {
                            irs.push(IR::ADD(1));
                        }
                    }
                }
                OpCode::SUB => {
                    match irs.last_mut() {
                        Some(IR::SUB(x)) => {
                            let (b, _) = x.overflowing_add(1);
                            *x = b;
                        }
                        _ => {
                            irs.push(IR::SUB(1));
                        }
                    }
                }
                OpCode::PUTCHAR => {
                    irs.push(IR::PUTCHAR);
                }
                OpCode::GETCHAR => {
                    irs.push(IR::GETCHAR);
                }
                OpCode::LB => {
                    irs.push(IR::JIZ(0));
                    jsp.push((irs.len() - 1) as u32)
                }
                OpCode::RB => {
                    let i: u32 = jsp.pop().ok_or("pop from empty stack")?;
                    irs.push(IR::JNE(i));
                    let irs_len = irs.len();
                    match &mut irs[i as usize] {
                        IR::JIZ(x) => {
                            *x = (irs_len - 1) as u32;
                        }
                        _ => unreachable!()
                    }
                }
            }
        }
        Ok(Code { instrs: irs })
    }
}
