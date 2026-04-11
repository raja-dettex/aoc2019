use crate::error::Error;
pub fn run<R>(mut input: R) -> std::result::Result<(), Error>
where R: std::io::BufRead { 
    let mut buf = String::new();
    let mut total = 0;
    let mut part_two_fuel = 0;
    loop { 
        if input.read_line(&mut buf)? == 0 { 
            break;
        }
        let mass = buf.trim().parse::<usize>()?;
        let fuel = part_one(mass);
        let fuel_of_fuel = part_two(fuel);
        total += fuel;
        part_two_fuel += (fuel + fuel_of_fuel);
        buf.clear();
    }
    println!("total fuel {}", total);
    println!("sum total fuel {}", part_two_fuel);
    Ok(())
}


pub fn part_one(mass: usize) -> usize { 
    match (mass / 3).checked_sub(2) { 
        Some(v) => v,
        None => 0
    }
}


pub fn part_two(mut fuel: usize) -> usize { 
    let mut s = 0;
    let total = loop { 
        let sub_fuel  = match (fuel / 3).checked_sub(2) { 
            Some(v) => v,
            None => break s
        };
        s += sub_fuel;
        fuel = sub_fuel;
    };
    return total;
}