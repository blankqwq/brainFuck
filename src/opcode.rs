use std::collections::HashMap;
#[derive(Debug)]
pub enum OpCode {
    SHR = 0x3E,
    SHL = 0x3C,
    ADD = 0x2B,
    SUB = 0x2D,
    PUTCHAR = 0x2E,
    GETCHAR = 0x2c,
    LB = 0x5B,
    RB = 0x5D,
}

impl From<u8> for OpCode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => OpCode::SHR,
            0x3C => OpCode::SHL,
            0x2B => OpCode::ADD,
            0x2D => OpCode::SUB,
            0x2E => OpCode::PUTCHAR,
            0x2c => OpCode::GETCHAR,
            0x5B => OpCode::LB,
            0x5D => OpCode::RB,
            _ => panic!()
        }
    }
}
#[warn(dead_code)]
#[derive(Debug)]
pub struct Code {
    pub instrs: Vec<OpCode>,
    pub jtables: HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let v = vec![
            OpCode::SHR as u8,
            OpCode::SHL as u8,
            OpCode::ADD as u8,
            OpCode::SUB as u8,
            OpCode::PUTCHAR as u8,
            OpCode::GETCHAR as u8,
            OpCode::LB as u8,
            OpCode::RB as u8,
        ];
        let instrs:Vec<OpCode> = data.iter()
            .filter(|x| v.contains(x))
            .map(|x| OpCode::from(*x))
            .collect();
        let mut jtables:HashMap<usize,usize> = HashMap::new();
        let mut temp:i32 = -1;
        for (i,v) in instrs.iter().enumerate(){
            match &v {
                OpCode::LB | OpCode::RB=>{
                    if temp>0 {
                        jtables.insert(temp as usize, i);
                        jtables.insert(i,temp as usize);
                        temp=-1
                    }else {
                        temp= i as i32;
                    }
                },
                _ => {}
            }
        }
        // println!("{:?}",jtables);
        Ok(Code{instrs, jtables })
    }
}