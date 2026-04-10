pub fn run<R>(mut input: R) 
where R: std::io::BufRead { 
    let mut buf = String::new();
    let mut total = 0;
    loop { 
        if input.read_line(&mut buf).unwrap() == 0 { 
            break;
        }
        let mass = buf.trim().parse::<usize>().unwrap();
        let fuel = match (mass / 3).checked_sub(2) { 
            Some(v) => v,
            None => 0
        };
        total += fuel;
        buf.clear();
    }
    println!("total fuel {}", total);
}