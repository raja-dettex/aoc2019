use clap::Parser;
use aoc2019::{self, bail, day1, day2, day3, error, reader};
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// no. of day
    #[arg(short, long)]
    day: usize,

    /// which file to read from; its optional or just pipe the stdin without the cli args; pretty simnple
    #[arg(short, long)]
    input: Option<String>,
} 
fn main() { 
    if let Err(e) = run() { 
        println!("{}", e);
    }
}
fn run() -> std::result::Result<(), error::Error>{
    let args = Args::parse(); 
    let mut r = match args.input { 
        Some(path) => {
            let file = std::fs::File::open(path).unwrap();
            let reader = std::io::BufReader::new(file); 
            reader::Reader::File(reader)
        },
        None => { 
            let stdin = std::io::stdin();
            let guard = stdin.lock();
            let reader = std::io::BufReader::new(guard);
            reader::Reader::Stdin(reader)
        }
    };
    let day = args.day;
    match day { 
        1 => day1::run(&mut r)?,
        2 => day2::run(&mut r)?,
        3 => day3::run(&mut r)?,
        n if n > 1 && n < 26 => bail!("day {} is not implemented yet ", day),
        _ => bail!("day must be between 1 and 26")
    }
    Ok(())
}
