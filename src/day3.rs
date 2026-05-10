use std::{collections::HashMap, io::Lines};

use atoi::atoi;

use crate::{bail, error::Error};

type State = HashMap<Point, [Option<u32>; 2]>;
const ORIGIN: Point = Point(0, 0);


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32);

#[derive(Clone, Copy)]
struct Instruction { 
    dir: Direction,
    dist: u32
}

impl std::str::FromStr for Instruction {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let bytes = value.as_bytes();
        let dir = Direction::try_from(bytes[0] as char)?;
        let dist = atoi::<u32>(&bytes[1..]).ok_or_else(|| crate::error!("unable to parse"))?;
        Ok(Self { 
            dir,
            dist
        })
    }
}

#[derive(Clone, Copy)]
enum Direction { 
    U,
    D,
    R,
    L
}

impl TryFrom<char> for Direction { 
    type Error = Error;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let res = match value { 
            'U' => Self::U,
            'D' => Self::D,
            'R' => Self::R,
            'L' => Self::L,
            _ => { 
                return crate::bail!("not implemented for this char");
            }
        };
        Ok(res)
    }
}

pub fn run<I>(input: I) -> Result<(), Error>
where I: std::io::BufRead
{
    let lines = input.lines();
    let mut id = 0;
    let mut state: State = HashMap::new();
    for line in lines { 
        assert!(id < 2);
        let line = line?;
        let mut point = ORIGIN;
        let mut steps = 0;
        let inst_list = line.split(",").collect::<Vec<&str>>();
        for inst_str in inst_list { 
            let inst = inst_str.parse::<Instruction>()?;
            let new_point = process_instruction(id, steps, point, inst, &mut state);
            point = new_point;
            steps += inst.dist; 
        }
        id += 1;
    }
    let (answer1, answer2) = state.iter().filter(|(_, v)| v[0].is_some() && v[1].is_some()).fold(
        (std::u32::MAX,  std::u32::MAX),  |(mut min_dist, mut min_steps), (point, steps)| {
        let dist = manhattan_distance(*point, ORIGIN);
        if dist < min_dist { 
            min_dist = dist;
        }
        let total_steps = steps[0].unwrap() + steps[1].unwrap();
        if total_steps < min_steps { 
            min_steps = total_steps;
        }
        (min_dist, min_steps)
    });
    println!("answer1 {}, answer2 : {}", answer1, answer2);
    Ok(())
}


fn manhattan_distance(a: Point, b: Point) -> u32 { 
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u32
} 

fn process_instruction(id: u32, steps: u32, origin: Point, instruction: Instruction, state: &mut State) -> Point { 
    let (x, y) = match instruction.dir {
        Direction::U => (0, 1),
        Direction::D => (0, -1),
        Direction::R => (1, 0),
        Direction::L => (-1, 0),
    };
    let mut destination = origin;
    for n in 1..=instruction.dist { 
        let point = Point(origin.0 + x * n as i32, origin.1 + y * n as i32);
        let value = state.entry(point).or_insert_with(|| [None, None]);
        if value[id as usize].is_none() { 
            value[id as usize] = Some(steps + n);
        }
        destination = point;
    }    
    destination
}
