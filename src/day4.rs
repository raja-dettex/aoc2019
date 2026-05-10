use crate::{error, error::Error};

pub fn run<R>(mut input: R) -> std::result::Result<(), Error>
where R: std::io::BufRead { 
    let (low, high) = read_input(input)?;
    println!("low {} and high {}", low, high);
    println!("{:?}", parse_digits(low as i32));
    let mut answer1 = 0;
    let mut answer2 = 0;
    'outer: for n in low..=high { 
        let digits = parse_digits(n as i32);
        let mut state = State::None;
        let mut min = 0;
        let mut is_valid = (false, false);
        for (i, digit) in digits.iter().enumerate() { 
            let digit = *digit;
            if digit < min { 
                continue 'outer;
            }
            match state {
                State::None => state = State::One(digit),
                State::One(d) => { 
                    if d == digit { 
                        is_valid.0 = true;
                        if i == 5 { is_valid.1 = true; }
                        state = State::Two(digit)
                    } else { 
                        state = State::One(digit)
                    }
                },
                State::Two(d)  => {
                    if d == digit { 
                        state = State::Three_Or_More(digit);
                    } else { 
                        is_valid.1 = true;
                        state = State::One(digit);
                    }
                },
                State::Three_Or_More(d) => { 
                    if d == digit  { 
                    } else { 
                        state = State::One(digit);
                    }
                }
            }
            min = digit;
        }
        
        
        if is_valid.0 { answer1 += 1; }
        if is_valid.1 { answer2 += 1; }
    }
    println!("{}, {}", answer1, answer2);
    Ok(())
}

enum State { 
    None,
    One(u8),
    Two(u8),
    Three_Or_More(u8)
}
fn parse_digits(n: i32) -> [u8; 6] {
    let mut out = [0u8; 6];
    let mut residual = n;
    for i in 0..6 {
        out[5 - i as usize] = (residual - (residual / 10) * 10) as u8;
        residual = residual / 10;
    }
    out
}


fn read_input<I>( mut input: I) -> Result<(usize, usize), Error>  
where I: std::io::BufRead 
{
    let parse = |s: &str| s.trim().parse::<usize>();
    let error = || error!("parse failed");
    let mut s = String::new(); 
    input.read_to_string(&mut s)?;
    let mut iter = s.split("-");
    let low = iter.next().map(parse).ok_or_else(error)??;
    let high = iter.next().map(parse).ok_or_else(error)??;
    Ok((low, high))
}
