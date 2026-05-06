use crate::{bail, error, error::Error};

pub struct Emulator { 
    vrom: Vec<usize>, 
    vram: Vec<usize>,
    pc: usize
}

pub fn run<R>(input: R) -> Result<(), Error> 
where R: std::io::BufRead 
{ 
    let mut emulator = Emulator::new(input)?;
    let ans = emulator.execute(Some(12), Some(2))?;
    println!("{}", ans);
    let upper_bound = 99 as usize;
    let mut answer: Result<usize, Error> = Err(error!("answer not found in the scope with the possible noun and verb combination"));
    for noun in 0..=upper_bound { 
        for verb in 0..=upper_bound { 
            if emulator.execute(Some(noun), Some(verb))? == 19690720 { 
                answer = Ok(noun * 100 + verb);
            }
        }
    }
    println!("{}", answer?);
    Ok(())
}


impl Emulator { 
    pub fn new<R>(mut input: R) -> Result<Self, Error>
    where R: std::io::BufRead
    { 
        let mut buffer = String::new();
        input.read_to_string(&mut buffer)?;
        let vrom = buffer.trim()
            .split(",").map(|s| Ok(s.parse::<usize>()?)).collect::<Result<Vec<_>, Error>>()?;
        Ok(Self { 
            vrom,
            vram: Vec::new(),
            pc: 0
        })
    }

    pub fn execute(&mut self, noun: Option<usize>, verb: Option<usize>) -> Result<usize, Error> {
        self.vram = self.vrom.clone();
        self.pc = 0;
        if let Some(n) = noun { self.vram[1] = n };
        if let Some(v) = verb { self.vram[2] = v };
        loop { 
            let opcode = self.vram[self.pc];
            match opcode { 
                1 | 2 => { 
                    let i1_ptr = self.vram[self.pc + 1];
                    let i2_ptr = self.vram[self.pc + 2];
                    let w_ptr = self.vram[self.pc + 3];
                    let i1 = self.vram[i1_ptr];
                    let i2 = self.vram[i2_ptr];
                    let out = if opcode == 1 { i1 + i2} else { i1 * i2};
                    self.vram[w_ptr] = out;
                    self.pc += 4;
                },
                99 => break,
                _ => bail!("invalid opcode {}", opcode)

            }
        }
        Ok(self.vram[0])
    }
}



#[cfg(test)] 
mod day2_tests {
    use std::io::BufReader;

    use crate::day2::Emulator;
 

    #[test]
    pub fn test_day2() { 
        let test_cases = &[
            // (input_mem, state_of_mutated_mem)
            ("1,0,0,0,99", "2,0,0,0,99"),
            ("2,3,0,3,99", "2,3,0,6,99"),
            ("2,4,4,5,99,0", "2,4,4,5,99,9801"),
            ("1,1,1,4,99,5,6,0,99", "30,1,1,4,2,5,6,0,99")
        ];

        for (input, final_state) in test_cases.to_owned() {
            let mut reader = BufReader::new(input.as_bytes());
            let mut emulator = Emulator::new(reader).unwrap();
            emulator.execute(None, None).unwrap();
            let state = emulator.vram.iter().map(|u| u.to_string()).collect::<Vec<_>>().join(",");
            println!("{} :  {}", final_state, state);       
            assert_eq!(final_state, state);
        }
    }
}