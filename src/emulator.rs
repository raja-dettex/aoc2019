use crate::{bail, error::Error};
pub(crate) struct Emulator { 
    vrom: Vec<i64>, 
    vram: Vec<i64>,
    pc: u64
}

impl Emulator { 
    pub(crate) fn new<R>(mut input: R) -> Result<Self, Error>
    where R: std::io::BufRead
    { 
        let mut buffer = String::new();
        input.read_to_string(&mut buffer)?;
        let vrom = buffer.trim()
            .split(",").map(|s| Ok(s.parse::<i64>()?)).collect::<Result<Vec<_>, Error>>()?;
        Ok(Self { 
            vrom,
            vram: Vec::new(),
            pc: 0
        })
    }

    pub(crate) fn execute(&mut self, noun: i64, verb: i64) -> Result<i64, Error> {
        self.vram = self.vrom.clone();
        self.pc = 0;
        self.vram[1] = noun;
        self.vram[2] = verb;
        loop { 
            let opcode = self.vram[self.pc as usize];
            match opcode { 
                1 | 2 => { 
                    let i1_ptr = self.vram[(self.pc + 1) as usize];
                    let i2_ptr = self.vram[(self.pc + 2) as usize];
                    let w_ptr = self.vram[(self.pc + 3) as usize];
                    let i1 = self.vram[i1_ptr as usize];
                    let i2 = self.vram[i2_ptr as usize];
                    let out = if opcode == 1 { i1 + i2} else { i1 * i2};
                    self.vram[w_ptr as usize] = out;
                    self.pc += 4;
                },
                99 => break,
                _ => bail!("invalid opcode {}", opcode)

            }
        }
        Ok(self.vram[0])
    }
    pub(crate) fn vram(&self) -> &[i64] { 
        &self.vram
    }
}



#[cfg(test)] 
mod day2_tests {
    use std::io::BufReader;

    use crate::emulator::Emulator;
 

    #[test]
    pub fn test_day2() { 
        let test_cases = &[
            // (input_mem, state_of_mutated_mem)
            ("1,0,0,0,99", 0, 0, "2,0,0,0,99"),
            ("2,3,0,3,99", 3, 0, "2,3,0,6,99"),
            ("2,4,4,5,99,0", 4, 4,"2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", 1, 1, "30,1,1,4,2,5,6,0,99")
        ];

        for (input, noun, verb, final_state) in test_cases.to_owned() {
            let mut reader = BufReader::new(input.as_bytes());
            let mut emulator = Emulator::new(reader).unwrap();
            emulator.execute(noun, verb).unwrap();
            let state = emulator.vram().iter().map(|u| u.to_string()).collect::<Vec<_>>().join(",");
            println!("{} :  {}", final_state, state);       
            assert_eq!(final_state, state);
        }
    }
}