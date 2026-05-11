use crate::{bail, error, error::Error, emulator::Emulator};

pub fn run<R>(input: R) -> Result<(), Error> 
where R: std::io::BufRead 
{ 
    let mut emulator = Emulator::new(input)?;
    let ans = emulator.execute(12, 2)?;
    println!("{}", ans);
    let upper_bound = 99i64;
    let mut answer: Result<i64, Error> = Err(error!("answer not found in the scope with the possible noun and verb combination"));
    for noun in 0..=upper_bound { 
        for verb in 0..=upper_bound { 
            if emulator.execute(noun, verb)? == 19690720 { 
                answer = Ok(noun * 100 + verb);
            }
        }
    }
    println!("{}", answer?);
    Ok(())
}

